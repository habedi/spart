## Feature Roadmap

This document includes the roadmap for the Spart project.
It outlines features to be implemented and their current status.

> [!IMPORTANT]
> This roadmap is a work in progress and is subject to change.

- **Core Data Structures**
    -   [x] Quadtree (2D)
    -   [x] Octree (3D)
    -   [x] Kd-tree (2D and 3D)
    -   [x] R-tree (2D and 3D)
    -   [x] R*-tree (2D and 3D)

- **Supported Geometries and Queries**
    -   [x] Point data (`Point2D` and `Point3D`)
    -   [x] kNN search
    -   [x] Circular or spherical range search
    -   [x] Rectangular and cuboid range search (`range_search_bbox`)
    -   [ ] `update` method for moving points (currently needs delete + insert)
    -   [ ] Support for storing non-point geometries (for example, lines, polygons)
    -   [ ] Advanced intersection queries (like finding all stored items that intersect a given polygon)

- **Performance and Optimization**
    -   [x] Bulk loading implementations for faster tree construction
    -   [ ] Thread-safety for concurrent reads (like `&Tree` accessible from multiple threads)
    -   [ ] Arena allocation for tree nodes to improve cache locality
    -   [ ] SIMD-accelerated distance and intersection calculations if possible

- **API and Developer Experience**
    -   [x] Simple API for tree creation and manipulation
    -   [x] Serialization and deserialization via `serde`
    -   [x] Custom distance metric support
    -   [ ] Public iterators for tree traversal (something like `tree.iter()`)
    -   [ ] Tree diagnostic methods (`height()`, `node_count()`, etc.)
    -   [ ] Replace internal panics with `Result`-based error handling (for example, for invalid dimensions)

- **Ecosystem and Bindings**
    -   [x] Python bindings (`pyspart`) for all tree types
    -   [ ] Full feature parity for Python bindings (like bulk loading for all trees)
    -   [ ] WebAssembly support for browser and serverless environments

- **Benchmarks**
    -   [x] Benchmarks for comparing the performance of tree implementations and operations
    -   [ ] Benchmarks for comparing the performance against other similar libraries
