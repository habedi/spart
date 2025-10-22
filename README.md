<div align="center">
  <picture>
    <img alt="Spart Logo" src="logo.svg" height="30%" width="30%">
  </picture>
<br>

<h2>Spart</h2>

[![Tests](https://img.shields.io/github/actions/workflow/status/habedi/spart/tests.yml?label=tests&style=flat&labelColor=282c34&logo=github)](https://github.com/habedi/spart/actions/workflows/tests.yml)
[![Code Coverage](https://img.shields.io/codecov/c/github/habedi/spart?label=coverage&style=flat&labelColor=282c34&logo=codecov)](https://codecov.io/gh/habedi/spart)
[![Code Quality](https://img.shields.io/codefactor/grade/github/habedi/spart?label=quality&style=flat&labelColor=282c34&logo=codefactor)](https://www.codefactor.io/repository/github/habedi/spart)
[![Crates.io](https://img.shields.io/crates/v/spart.svg?label=crates.io&style=flat&labelColor=282c34&color=fc8d62&logo=rust)](https://crates.io/crates/spart)
[![Docs.rs](https://img.shields.io/badge/docs-spart-66c2a5?style=flat&labelColor=282c34&logo=docs.rs)](https://docs.rs/spart)
[![MSRV](https://img.shields.io/badge/msrv-1.83.0-informational?style=flat&labelColor=282c34&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-007ec6?style=flat&labelColor=282c34&logo=open-source-initiative)](https://github.com/habedi/spart)

A collection of space partitioning trees for Rust

</div>

---

Spart (**spa**ce **par**titioning **t**rees) is a Rust library that provides implementations of several
common [space partitioning tree data structures](https://en.wikipedia.org/wiki/Space_partitioning) that can be used for
indexing 2D and 3D point data to perform fast spatial queries, like k-nearest neighbor (kNN) and range search.

The library also includes Python bindings (see [pyspart](pyspart)), so it can easily be used in Python applications.

At the moment, the following tree data structures and features are supported:

| # | Tree Type                                          | 2D | 3D | kNN Search | Radius Search |
|---|----------------------------------------------------|:--:|:--:|:----------:|:-------------:|
| 1 | [Quadtree](https://en.wikipedia.org/wiki/Quadtree) | ✓  |    |     ✓      |       ✓       |
| 2 | [Octree](https://en.wikipedia.org/wiki/Octree)     |    | ✓  |     ✓      |       ✓       |
| 3 | [Kd-tree](https://en.wikipedia.org/wiki/K-d_tree)  | ✓  | ✓  |     ✓      |       ✓       |
| 4 | [R-tree](https://en.wikipedia.org/wiki/R-tree)     | ✓  | ✓  |     ✓      |       ✓       |
| 5 | [R*-tree](https://en.wikipedia.org/wiki/R*-tree)   | ✓  | ✓  |     ✓      |       ✓       |

See the [ROADMAP.md](ROADMAP.md) for the list of implemented and planned features.

> [!IMPORTANT]
> Spart is in early development, so bugs and breaking changes are expected.
> Please use the [issues page](https://github.com/habedi/spart/issues) to report bugs or request features.

---

### Installation

```bash
cargo add spart
````

*Spart requires Rust 1.83.0 or later.*

#### Python Bindings

You can install the Python bindings for Spart using `pip`:

```shell
pip install pyspart
```

Check out the [pyspart](pyspart) directory for more information about using Spart from Python.

---

### Documentation

For the Rust API documentation, see [docs.rs/spart](https://docs.rs/spart).

#### Basic Concepts

The basic building blocks of Spart are **point** and **tree**.

##### Point

A point is a tuple of coordinates plus an optional data payload of any type.
There are two types of points: `Point2D` and `Point3D`.

Example of 2D and 3D points:

```rust
use spart::geometry::{Point2D, Point3D};

fn main() {
    // There are two ways to create a point.

    // 1. Using the `new` method:
    let point_2d = Point2D::new(1.0, 2.0, Some("A 2D Point"));
    let point_3d = Point3D::new(1.0, 2.0, 3.0, Some("A 3D Point"));

    // 2. Using a struct literal:
    let point_2d_literal = Point2D {
        x: 1.0,
        y: 2.0,
        data: Some("A 2D Point"),
    };
    let point_3d_literal = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("A 3D Point"),
    };
}
```

##### Tree

A tree is a spatial data structure that indexes points and provides methods for querying them.

Currently, the following trees are implemented:

- Quadtree (2D)
- Octree (3D)
- Kd-tree (2D and 3D)
- R-tree (2D and 3D)
- R*-tree (2D and 3D)

A tree provides at least the following methods:

- `new`: creates a new tree given the following parameters:
    - The bounding area of the tree (for Quadtree and Octree only)
    - The number of dimensions (for Kd-tree only)
    - The maximum capacity of points per node (for Quadtree, Octree, and R-tree)
- `insert`: inserts a point into the tree.
- `insert_bulk`: inserts multiple points into the tree at once.
    - This is generally more efficient than inserting points one by one.
- `delete`: removes a point from the tree.
- `knn_search`: finds the k nearest neighbors to a query point.
    - The inputs are the query point and the number of neighbors to find.
- `range_search`: finds all points within a given range of a query point.
    - The inputs are the query point and the range within which to search.

> [!NOTE]
> Currently, the following properties hold for all trees:
> - Duplicates are allowed: inserting a duplicate point will add another copy to the tree.
> - Searches return duplicates: both `knn_search` and `range_search` can return duplicate points if they were previously
    inserted.
> - Deletion removes one instance: if there are duplicate points, the `delete` operation removes only one instance of
    the point from the tree.
> - A `knn_search` with `k=0` will return an empty list.
> - A `knn_search` with `k` greater than the number of points in the tree will return all points.
> - A `range_search` with a radius of `0` will return only points with the exact same coordinates.
>
> The distance metric used for nearest neighbor and range searches is the Euclidean distance by default.
> However, you can use a custom distance metric by implementing the `DistanceMetric` trait.
>
> For example, here is how you can define and use the Manhattan distance:
> ```rust
> use spart::geometry::{Point2D, DistanceMetric};
>
> // 1. Define a struct for your distance metric.
> struct ManhattanDistance;
>
> // 2. Implement the `DistanceMetric` trait for your point type.
> impl<T> DistanceMetric<Point2D<T>> for ManhattanDistance {
>     fn distance_sq(p1: &Point2D<T>, p2: &Point2D<T>) -> f64 {
>         ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()).powi(2)
>     }
> }
>
> // 3. Use it in a search function.
> // tree.knn_search::<ManhattanDistance>(&query_point, 1);
> ```

#### Serialization

Spart trees can be serialized and deserialized using the `serde` feature.

To enable serialization in Rust, you need to enable the `serde` feature in your `Cargo.toml` file:

```toml
[dependencies]
spart = { version = "0.3.0", features = ["serde"] }
```

Then, you can use `bincode` (or any other serde-compatible library) to serialize and deserialize the tree.
For example, you can save and load a tree to and from a file:

```rust
use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut qt = Quadtree::new(&boundary, 4).unwrap();
    qt.insert(Point2D::new(10.0, 20.0, Some("point1".to_string())));
    qt.insert(Point2D::new(50.0, 50.0, Some("point2".to_string())));

    // Serialize the tree to a file
    let encoded: Vec<u8> = bincode::serialize(&qt).unwrap();
    let mut file = File::create("tree.spart").unwrap();
    file.write_all(&encoded).unwrap();

    // Deserialize the tree from a file
    let mut file = File::open("tree.spart").unwrap();
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).unwrap();
    let decoded: Quadtree<String> = bincode::deserialize(&encoded[..]).unwrap();
}
```

#### Debugging Mode

You can enable debugging mode for Spart by setting the `DEBUG_SPART` environment variable to `true` or `1`.

```bash
# Enable debugging mode on Linux and macOS
export DEBUG_SPART=true
```

```powershell
# Enable debugging mode on Windows (PowerShell)
$env:DEBUG_SPART = "true"
```

> [!NOTE]
> When debugging mode is enabled, Spart will be very verbose.
> It is recommended to use this only for debugging purposes.

### Examples

- For Rust examples, see the [examples](examples) directory.
- For Python examples, see [pyspart/examples](pyspart/examples).

---

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to make a contribution.

### License

Spart is available under the terms of either of the following licenses:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

### Acknowledgements

* The logo is from [SVG Repo](https://www.svgrepo.com/svg/382456/autumn-fall-leaf-orange-season-tree).
