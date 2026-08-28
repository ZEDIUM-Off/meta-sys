#![feature(rustc_private)]
#![warn(unused_extern_crates)]

//! Structural readability lint for Meta-System Rust code.
//!
//! The compiler already exposes strong semantic lints, while rustfmt and Clippy cover line width,
//! function size, arguments, and complexity. This lint keeps the related project policy in one
//! diagnostic surface and adds the file-level limits those tools do not provide.

extern crate rustc_hir;
extern crate rustc_span;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, FileName, Span, SyntaxContext};
use serde::Deserialize;

dylint_linting::impl_late_lint! {
    /// ### What it does
    ///
    /// Enforces configured limits on function lines, function arguments, functions per source
    /// file, and source-file lines.
    ///
    /// ### Why is this bad?
    ///
    /// Large functions and files hide responsibilities, increase review cost, and make it harder to
    /// recover a module's invariants in one scan. Long parameter lists usually indicate a missing
    /// domain type or configuration object.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn activate(a: A, b: B, c: C, d: D, e: E, f: F) {
    ///     // a long sequence of unrelated steps
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// struct Activation { /* named inputs */ }
    ///
    /// fn activate(input: Activation) {
    ///     validate(&input);
    ///     apply(input);
    /// }
    /// ```
    ///
    /// ### Configuration
    ///
    /// Configure the four `usize` thresholds under `[meta_sys_style]` in `dylint.toml`:
    /// `max_function_lines`, `max_function_arguments`, `max_functions_per_file`, and
    /// `max_file_lines`.
    pub META_SYS_STYLE,
    Deny,
    "Rust structure exceeds the project's scan-friendly limits",
    MetaSysStyle::new()
}

/// Thresholds loaded from the linted workspace's `dylint.toml`.
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Config {
    /// Maximum physical lines covered by a function span.
    max_function_lines: usize,
    /// Maximum parameters in a function, including a method receiver.
    max_function_arguments: usize,
    /// Maximum functions and methods declared in one source file.
    max_functions_per_file: usize,
    /// Maximum physical lines in one source file.
    max_file_lines: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_function_lines: 50,
            max_function_arguments: 5,
            max_functions_per_file: 12,
            max_file_lines: 400,
        }
    }
}

/// Facts collected for one real source file during the late lint pass.
struct FileStats {
    /// Span at the beginning of the file for file-level diagnostics.
    start_span: Span,
    /// Number of physical lines present in the loaded source.
    line_count: usize,
    /// Spans of functions and methods in declaration order.
    functions: Vec<Span>,
}

/// Stateful lint pass that joins function-level and file-level observations.
struct MetaSysStyle {
    /// Active limits for the linted workspace.
    config: Config,
    /// Source facts indexed by the local path reported by `rustc`.
    files: BTreeMap<PathBuf, FileStats>,
}

impl MetaSysStyle {
    /// Creates a pass from the workspace configuration.
    fn new() -> Self {
        Self {
            config: dylint_linting::config_or_default(env!("CARGO_PKG_NAME")),
            files: BTreeMap::new(),
        }
    }

    /// Records a function span against its real source file when one exists.
    fn record_function(&mut self, cx: &LateContext<'_>, span: Span) {
        let source_file = cx.sess().source_map().lookup_source_file(span.lo());
        let Some(path) = local_path(&source_file.name) else {
            return;
        };

        if let Some(stats) = self.files.get_mut(path) {
            stats.functions.push(span);
        }
    }

    /// Emits the function-size diagnostic when its span crosses the configured boundary.
    fn check_function_lines(&self, cx: &LateContext<'_>, span: Span) {
        let Some(line_count) = span_line_count(cx, span) else {
            return;
        };

        if line_count > self.config.max_function_lines {
            span_lint_and_help(
                cx,
                META_SYS_STYLE,
                span,
                format!(
                    "function has {line_count} lines; the limit is {}",
                    self.config.max_function_lines
                ),
                None,
                "extract named steps or move a responsibility into a focused type",
            );
        }
    }

    /// Emits the parameter-count diagnostic when the signature crosses the configured boundary.
    fn check_function_arguments(&self, cx: &LateContext<'_>, declaration: &FnDecl<'_>, span: Span) {
        let argument_count = declaration.inputs.len();
        if argument_count > self.config.max_function_arguments {
            span_lint_and_help(
                cx,
                META_SYS_STYLE,
                span,
                format!(
                    "function has {argument_count} arguments; the limit is {}",
                    self.config.max_function_arguments
                ),
                None,
                "introduce a cohesive input type or split the operation",
            );
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for MetaSysStyle {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        for file in cx.sess().source_map().files().iter() {
            let Some(path) = local_path(&file.name) else {
                continue;
            };
            let Some(source) = file.src.as_deref() else {
                continue;
            };

            let span_end = if source.is_empty() {
                file.start_pos
            } else {
                file.start_pos + BytePos(1)
            };
            let start_span = Span::new(file.start_pos, span_end, SyntaxContext::root(), None);
            self.files.entry(path.to_path_buf()).or_insert(FileStats {
                start_span,
                line_count: source.lines().count(),
                functions: Vec::new(),
            });
        }
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        declaration: &'tcx FnDecl<'tcx>,
        _body: &'tcx Body<'tcx>,
        span: Span,
        _definition: LocalDefId,
    ) {
        if matches!(kind, FnKind::Closure) || span.from_expansion() {
            return;
        }

        self.record_function(cx, span);
        self.check_function_lines(cx, span);
        self.check_function_arguments(cx, declaration, span);
    }

    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        for stats in self.files.values() {
            if stats.line_count > self.config.max_file_lines {
                span_lint_and_help(
                    cx,
                    META_SYS_STYLE,
                    stats.start_span,
                    format!(
                        "source file has {} lines; the limit is {}",
                        stats.line_count, self.config.max_file_lines
                    ),
                    None,
                    "split the file along a named responsibility or invariant",
                );
            }

            if stats.functions.len() > self.config.max_functions_per_file {
                let first_excess = stats.functions[self.config.max_functions_per_file];
                span_lint_and_help(
                    cx,
                    META_SYS_STYLE,
                    first_excess,
                    format!(
                        "source file has {} functions; the limit is {}",
                        stats.functions.len(),
                        self.config.max_functions_per_file
                    ),
                    None,
                    "move a cohesive group of functions into a focused module or type",
                );
            }
        }
    }
}

/// Returns the on-disk path for compiler inputs backed by a real local file.
fn local_path(file_name: &FileName) -> Option<&Path> {
    match file_name {
        FileName::Real(real_file_name) => real_file_name.local_path(),
        _ => None,
    }
}

/// Counts the physical lines covered by a span without crossing source files.
fn span_line_count(cx: &LateContext<'_>, span: Span) -> Option<usize> {
    let source_map = cx.sess().source_map();
    let start = source_map.lookup_char_pos(span.lo());
    let inclusive_end = if span.hi() > span.lo() {
        span.hi() - BytePos(1)
    } else {
        span.hi()
    };
    let end = source_map.lookup_char_pos(inclusive_end);

    (start.file.name == end.file.name).then(|| end.line.saturating_sub(start.line) + 1)
}

#[test]
fn ui() {
    dylint_testing::ui::Test::src_base(env!("CARGO_PKG_NAME"), "ui")
        .dylint_toml(
            r#"
            meta_sys_style.max_function_lines = 5
            meta_sys_style.max_function_arguments = 2
            meta_sys_style.max_functions_per_file = 2
            meta_sys_style.max_file_lines = 20
            "#,
        )
        .run();
}
