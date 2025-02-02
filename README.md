# Space Partitioning Trees for Rust 🦀

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="assets/logos/rustacean-flat-happy.svg">
    <source media="(prefers-color-scheme: dark)" srcset="assets/logos/rustacean-flat-happy.svg">
    <img alt="spart logo" src="assets/logos/rustacean-flat-happy.svg" height="40%" width="40%">
  </picture>
</div>
<br>

<p align="center">
  <a href="https://github.com/habedi/spart/actions/workflows/tests.yml">
    <img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/habedi/spart/tests.yml?label=Tests&style=flat&labelColor=555555&logo=github">
  </a>
  <a href="docs">
    <img alt="Docs" src="https://img.shields.io/badge/docs-latest-3776ab?style=flat&labelColor=555555&logo=readthedocs">
  </a>
  <a href="https://github.com/habedi/spart">
    <img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&labelColor=555555&logo=open-source-initiative">
  </a>
  <a href="https://codecov.io/gh/habedi/spart">
    <img alt="Code Coverage" src="https://img.shields.io/codecov/c/github/habedi/spart?style=flat&labelColor=555555&logo=codecov">
  </a>
  <a href="https://www.codefactor.io/repository/github/habedi/spart">
    <img alt="CodeFactor" src="https://img.shields.io/codefactor/grade/github/habedi/spart?style=flat&labelColor=555555&logo=codefactor">
  </a>
  <a href="https://crates.io/crates/spart">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/spart.svg?style=flat&color=fc8d62&logo=rust">
  </a>
  <a href="https://docs.rs/spart">
    <img alt="Docs.rs" src="https://img.shields.io/badge/docs.rs-quadtree--zng--66c2a5?style=flat&labelColor=555555&logo=docs.rs">
  </a>
  <a href="https://crates.io/crates/spart">
    <img alt="Downloads" src="https://img.shields.io/crates/d/spart?style=flat&labelColor=555555&logo=rust">
  </a>
</p>

Spart (**s**[pace] **par**[titioning] **t**[rees] is a collection of space partitioning tree data structures
implemented in Rust.

## Implementation Status

| Index | Tree Structure | 2D | 3D | Point | Range query | kNN query |
|-------|----------------|----|----|-------|-------------|-----------|
| 1     | Quad-tree      | ✓  |    | ✓     | ✓           | ✓         |
| 2     | Octree         |    | ✓  | ✓     | ✓           | ✓         |
| 3     | Kd-tree        | ✓  | ✓  | ✓     | ✓           | ✓         |
| 4     | R-tree         | ✓  | ✓  | ✓     | ✓           | ✓         |
| 5     | BSP-tree       | ✓  | ✓  | ✓     | ✓           | ✓         |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

## License

spart is licensed under either of these:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
