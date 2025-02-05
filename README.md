# Space Partitioning Trees for Rust 🦀

[<img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/habedi/spart/tests.yml?label=Tests&style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/habedi/spart/actions/workflows/tests.yml)
[<img alt="Code Coverage" src="https://img.shields.io/codecov/c/github/habedi/spart?style=for-the-badge&labelColor=555555&logo=codecov" height="20">](https://codecov.io/gh/habedi/spart)
[<img alt="CodeFactor" src="https://img.shields.io/codefactor/grade/github/habedi/spart?style=for-the-badge&labelColor=555555&logo=codefactor" height="20">](https://www.codefactor.io/repository/github/habedi/spart)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/spart.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/spart)
[<img alt="Docs.rs" src="https://img.shields.io/badge/docs.rs-spart-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/spart)
[<img alt="Downloads" src="https://img.shields.io/crates/d/spart?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/spart)
[<img alt="Docs" src="https://img.shields.io/badge/docs-latest-3776ab?style=for-the-badge&labelColor=555555&logo=readthedocs" height="20">](docs)
[<img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=for-the-badge&labelColor=555555&logo=open-source-initiative" height="20">](https://github.com/habedi/spart)

Spart (**s**[pace] **par**[titioning] **t**[rees] is a collection of space partitioning tree data structures
implemented in Rust.
It supports indexing and querying of point data in 2D and 3D spaces.

Currently, the following trees are implemented:

| Index | Tree Type | 2D | 3D | kNN query | Range query |
|-------|-----------|----|----|-----------|-------------|
| 1     | Quad-tree | ✓  |    | ✓         | ✓           |
| 2     | Octree    |    | ✓  | ✓         | ✓           |
| 3     | Kd-tree   | ✓  | ✓  | ✓         | ✓           |
| 4     | R-tree    | ✓  | ✓  | ✓         | ✓           |

*Version requirement: Spart supports rustc 1.83 and newer.*

[Release notes](https://github.com/habedi/spart/releases)

## Documentation

The documentation for the latest release can be found [here](docs).
Check out [examples](docs/examples) directory for usage examples.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

## License

Spart is available under the terms of either of the following licenses:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
