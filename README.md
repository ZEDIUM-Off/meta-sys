# Meta-system

Meta-system est un framework Rust de composition statique. Des Addons versionnés déclarent les
Capabilities qu’ils exposent, importent obligatoirement ou utilisent optionnellement; une System
Definition est validée directement pendant la compilation.

```text
A provides X
B requires X, provides Y
C requires Y
```

Le projet est actuellement revenu à sa phase de conception d’interface. Aucun crate produit n’est
actif tant que la forme Rust minimale de `Capability`, `Addon Contract` et `System` n’est pas
validée.

## Documentation

- [Vocabulaire canonique](CONTEXT.md)
- [Vision du projet](docs/vision.md)
- [Direction de la composition](docs/composition.md)
- [Protocole TDD Rust](docs/development/rust-tdd.md)
- [Qualité Rust](docs/development/rust-quality.md)
- [Audit historique du prototype runtime](docs/archive/runtime-prototype-audit.md)

## Tooling

```bash
make help
make check
```

`make check` valide le tooling Dylint même lorsque le workspace ne contient encore aucun crate
produit.
