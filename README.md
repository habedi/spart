## Spart

[<img alt="tests" src="https://img.shields.io/github/actions/workflow/status/habedi/spart/tests.yml?label=tests&style=flat&logo=github" height="20">](https://github.com/habedi/spart/actions/workflows/tests.yml)
[<img alt="code coverage" src="https://img.shields.io/codecov/c/github/habedi/spart?style=flat&logo=codecov" height="20">](https://codecov.io/gh/habedi/spart)
[<img alt="codefactor" src="https://img.shields.io/codefactor/grade/github/habedi/spart?style=flat&logo=codefactor" height="20">](https://www.codefactor.io/repository/github/habedi/spart)
[<img alt="crates.io" src="https://img.shields.io/crates/v/spart.svg?label=crates.io&style=flat&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/spart)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-spart-66c2a5?label=docs.rs&style=flat&logo=docs.rs" height="20">](https://docs.rs/spart)
[<img alt="msrv" src="https://img.shields.io/badge/msrv-1.83.0-informational?style=flat&logo=rust" height="20">](https://www.rust-lang.org)
[<img alt="license" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?label=license&style=flat&logo=open-source-initiative" height="20">](https://github.com/habedi/spart)

Spart (**s**[pace] **par**[titioning] **t**[rees] is a Rust library that provides implementations of popular
space partitioning tree data structures for efficient indexing and searching 2D and 3D point data.

Currently, the following trees are implemented:

| Index | Tree Type                                          | 2D | 3D | kNN Search | Range Search |
|-------|----------------------------------------------------|----|----|------------|--------------|
| 1     | [Quadtree](https://en.wikipedia.org/wiki/Quadtree) | ✓  |    | ✓          | ✓            |
| 2     | [Octree](https://en.wikipedia.org/wiki/Octree)     |    | ✓  | ✓          | ✓            |
| 3     | [Kd-tree](https://en.wikipedia.org/wiki/K-d_tree)  | ✓  | ✓  | ✓          | ✓            |
| 4     | [R-tree](https://en.wikipedia.org/wiki/R-tree)     | ✓  | ✓  | ✓          | ✓            |

### Installation

```bash
cargo add spart
```

*Spart requires Rust 1.83.0 or later.*

### Documentation

The documentation for the latest release can be found [here](docs).

Additionally, check out the [tests](tests/) directory for detailed examples for how to use the library.

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

### License

Spart is available under the terms of either of the following licenses:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
