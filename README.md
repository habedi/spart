## Spart

[<img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/habedi/spart/tests.yml?label=Tests&style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/habedi/spart/actions/workflows/tests.yml)
[<img alt="Lints" src="https://img.shields.io/github/actions/workflow/status/habedi/spart/lints.yml?label=Lints&style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/habedi/spart/actions/workflows/lints.yml)
[<img alt="Code Coverage" src="https://img.shields.io/codecov/c/github/habedi/spart?style=for-the-badge&labelColor=555555&logo=codecov" height="20">](https://codecov.io/gh/habedi/spart)
[<img alt="CodeFactor" src="https://img.shields.io/codefactor/grade/github/habedi/spart?style=for-the-badge&labelColor=555555&logo=codefactor" height="20">](https://www.codefactor.io/repository/github/habedi/spart)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/spart.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/spart)
[<img alt="Docs.rs" src="https://img.shields.io/badge/docs.rs-spart-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/spart)
[<img alt="Downloads" src="https://img.shields.io/crates/d/spart?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/spart)
[<img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.83.0-orange?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://github.com/rust-lang/rust/releases/tag/1.83.0)
[<img alt="Docs" src="https://img.shields.io/badge/docs-latest-3776ab?style=for-the-badge&labelColor=555555&logo=readthedocs" height="20">](docs)
[<img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=for-the-badge&labelColor=555555&logo=open-source-initiative" height="20">](https://github.com/habedi/spart)

Spart (**s**[pace] **par**[titioning] **t**[rees] is a Rust library that provides implementations of various
space partitioning tree data structures for efficient indexing and searching 2D and 3D point data.

Currently, the following trees are implemented:

| Index | Tree Type                                          | 2D | 3D | kNN Search | Range Search |
|-------|----------------------------------------------------|----|----|------------|--------------|
| 1     | [Quadtree](https://en.wikipedia.org/wiki/Quadtree) | ✓  |    | ✓          | ✓            |
| 2     | [Octree](https://en.wikipedia.org/wiki/Octree)     |    | ✓  | ✓          | ✓            |
| 3     | [Kd-tree](https://en.wikipedia.org/wiki/K-d_tree)  | ✓  | ✓  | ✓          | ✓            |
| 4     | [R-tree](https://en.wikipedia.org/wiki/R-tree)     | ✓  | ✓  | ✓          | ✓            |
| 5     | [BSP-tree](https://en.wikipedia.org/wiki/BSP-tree) | ✓  | ✓  | ✓          | ✓            |

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
