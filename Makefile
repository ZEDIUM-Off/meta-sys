.DEFAULT_GOAL := help

.PHONY: help check test test-doc test-package test-one

help: ## List available project commands.
	@awk 'BEGIN {FS = ":.*## "}; /^[a-zA-Z0-9_-]+:.*## / {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

check: ## Run the complete Rust quality gate.
	@./scripts/check-rust.sh

test: ## Run every Rust test suite.
	@./scripts/test-rust.sh all

test-doc: ## Run doctests for PACKAGE=<crate>.
	@./scripts/test-rust.sh doc "$(PACKAGE)"

test-package: ## Run all tests for PACKAGE=<crate>.
	@./scripts/test-rust.sh package "$(PACKAGE)"

test-one: ## Run exact TEST=<path> in PACKAGE=<crate>.
	@./scripts/test-rust.sh one "$(PACKAGE)" "$(TEST)"
