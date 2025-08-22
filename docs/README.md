## Spart Documentation

The basic building blocks of Spart are *point* and *tree*.

### Point

A point is a tuple of coordinates plus an optional data payload of any type.
There are two types of points: `Point2D` and `Point3D`.

Example of 2D and 3D points:

```rust
use spart::geometry::{Point2D, Point3D};

fn main() {
    // 2D point with coordinates (1.0, 2.0) and data "A 2D Point".
    let point_2d = Point2D {
        x: 1.0,
        y: 2.0,
        data: Some("A 2D Point"),
    };

    // 3D point with coordinates (1.0, 2.0, 3.0) and data "A 3D Point".
    let point_3d = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("A 3D Point"),
    };
}
```

### Tree

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
> - Searches return duplicates: both knn_search and range_search can return duplicate points if they were previously
    inserted.
> - Deletion removes duplicates: the delete operation removes all instances of the point from the tree.
>
> The distance metric used for nearest neighbor and range searches is the Euclidean distance.

### Examples

Below are some examples of how to use the different trees in Spart.
Check out the [tests](../tests) directory for more detailed examples.

#### Quadtree (2D)

```rust
use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;

fn main() {
    // Define the bounding area for the Quadtree.
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    // Create a new Quadtree with a maximum capacity of 3 points per node.
    let mut tree = Quadtree::new(&boundary, 3);

    // Define some 2D points.
    let point1 = Point2D {
        x: 1.0,
        y: 2.0,
        data: Some("Point1"),
    };
    let point2 = Point2D {
        x: 3.0,
        y: 4.0,
        data: Some("Point2"),
    };
    let point3 = Point2D {
        x: 5.0,
        y: 6.0,
        data: Some("Point3"),
    };
    let point4 = Point2D {
        x: 7.0,
        y: 8.0,
        data: Some("Point4"),
    };
    let point5 = Point2D {
        x: 2.0,
        y: 3.0,
        data: Some("Point5"),
    };

    // Insert points into the Quadtree.
    tree.insert(point1.clone());
    tree.insert(point2);
    tree.insert(point3);
    tree.insert(point4);
    tree.insert(point5);

    // Perform a k-nearest neighbor (kNN) search.
    let neighbors = tree.knn_search(&point1, 2);
    println!("kNN search results for {:?}: {:?}", point1, neighbors);

    // Perform a range search with a radius of 5.0.
    let range_points = tree.range_search(&point1, 5.0);
    println!("Range search results for {:?}: {:?}", point1, range_points);

    // Remove a point from the tree.
    tree.delete(&point1);
}
```

#### R*-tree (3D)

```rust
use spart::geometry::Point3D;
use spart::r_star_tree::RStarTree;

fn main() {
    // Create a new R*-tree with a maximum capacity of 4 points per node.
    let mut tree = RStarTree::new(4);

    // Define some 3D points.
    let point1 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("Point1"),
    };
    let point2 = Point3D {
        x: 3.0,
        y: 4.0,
        z: 5.0,
        data: Some("Point2"),
    };
    let point3 = Point3D {
        x: 5.0,
        y: 6.0,
        z: 7.0,
        data: Some("Point3"),
    };
    let point4 = Point3D {
        x: 7.0,
        y: 8.0,
        z: 9.0,
        data: Some("Point4"),
    };
    let point5 = Point3D {
        x: 2.0,
        y: 3.0,
        z: 4.0,
        data: Some("Point5"),
    };

    // Insert points into the R*-tree.
    tree.insert(point1.clone());
    tree.insert(point2);
    tree.insert(point3);
    tree.insert(point4);
    tree.insert(point5);

    // Perform a kNN search.
    let neighbors = tree.knn_search(&point1, 2);
    println!("kNN search results for {:?}: {:?}", point1, neighbors);

    // Perform a range search with a radius of 5.0.
    let range_points = tree.range_search(&point1, 5.0);
    println!("Range search results for {:?}: {:?}", point1, range_points);

    // Remove a point from the tree.
    tree.delete(&point1);
}
```

#### Octree (3D)

```rust
use spart::geometry::{Cube, Point3D};
use spart::octree::Octree;

fn main() {
    // Define the bounding area for the Octree.
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };

    // Create a new Octree with a maximum capacity of 3 points per node.
    let mut tree = Octree::new(&boundary, 3);

    // Define some 3D points.
    let point1 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("Point1"),
    };
    let point2 = Point3D {
        x: 3.0,
        y: 4.0,
        z: 5.0,
        data: Some("Point2"),
    };
    let point3 = Point3D {
        x: 5.0,
        y: 6.0,
        z: 7.0,
        data: Some("Point3"),
    };
    let point4 = Point3D {
        x: 7.0,
        y: 8.0,
        z: 9.0,
        data: Some("Point4"),
    };
    let point5 = Point3D {
        x: 2.0,
        y: 3.0,
        z: 4.0,
        data: Some("Point5"),
    };

    // Insert points into the Octree.
    tree.insert(point1.clone());
    tree.insert(point2);
    tree.insert(point3);
    tree.insert(point4);
    tree.insert(point5);

    // Perform a kNN search.
    let neighbors = tree.knn_search(&point1, 2);
    println!("kNN search results for {:?}: {:?}", point1, neighbors);

    // Perform a range search with a radius of 5.0.
    let range_points = tree.range_search(&point1, 5.0);
    println!("Range search results for {:?}: {:?}", point1, range_points);

    // Remove a point from the tree.
    tree.delete(&point1);
}
```

#### Kd-tree (3D)

```rust
use spart::geometry::Point3D;
use spart::kd_tree::KdTree;

fn main() {
    // Create a new Kd-tree for 3D points.
    let mut tree = KdTree::new(3);

    // Define some 3D points.
    let point1 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("Point1"),
    };
    let point2 = Point3D {
        x: 3.0,
        y: 4.0,
        z: 5.0,
        data: Some("Point2"),
    };
    let point3 = Point3D {
        x: 5.0,
        y: 6.0,
        z: 7.0,
        data: Some("Point3"),
    };
    let point4 = Point3D {
        x: 7.0,
        y: 8.0,
        z: 9.0,
        data: Some("Point4"),
    };
    let point5 = Point3D {
        x: 2.0,
        y: 3.0,
        z: 4.0,
        data: Some("Point5"),
    };

    // Insert points into the Kd-tree.
    tree.insert(point1.clone());
    tree.insert(point2);
    tree.insert(point3);
    tree.insert(point4);
    tree.insert(point5);

    // Perform a kNN search.
    let neighbors = tree.knn_search(&point1, 2);
    println!("kNN search results for {:?}: {:?}", point1, neighbors);

    // Perform a range search with a radius of 5.0.
    let range_points = tree.range_search(&point1, 5.0);
    println!("Range search results for {:?}: {:?}", point1, range_points);

    // Remove a point from the tree.
    tree.delete(&point1);
}
```

#### R-tree (3D)

```rust
use spart::geometry::Point3D;
use spart::r_tree::RTree;

fn main() {
    // Create a new R-tree with a maximum capacity of 4 points per node.
    let mut tree = RTree::new(4);

    // Define some 3D points.
    let point1 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        data: Some("Point1"),
    };
    let point2 = Point3D {
        x: 3.0,
        y: 4.0,
        z: 5.0,
        data: Some("Point2"),
    };
    let point3 = Point3D {
        x: 5.0,
        y: 6.0,
        z: 7.0,
        data: Some("Point3"),
    };
    let point4 = Point3D {
        x: 7.0,
        y: 8.0,
        z: 9.0,
        data: Some("Point4"),
    };
    let point5 = Point3D {
        x: 2.0,
        y: 3.0,
        z: 4.0,
        data: Some("Point5"),
    };

    // Insert points into the R-tree.
    tree.insert(point1.clone());
    tree.insert(point2);
    tree.insert(point3);
    tree.insert(point4);
    tree.insert(point5);

    // Perform a kNN search.
    let neighbors = tree.knn_search(&point1, 2);
    println!("kNN search results for {:?}: {:?}", point1, neighbors);

    // Perform a range search with a radius of 5.0.
    let range_points = tree.range_search(&point1, 5.0);
    println!("Range search results for {:?}: {:?}", point1, range_points);

    // Remove a point from the tree.
    tree.delete(&point1);
}
```

#### `insert_bulk`

```rust
use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;

fn main() {
    // Define the bounding area for the Quadtree.
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    // Create a new Quadtree with a maximum capacity of 10 points per node.
    let mut tree = Quadtree::new(&boundary, 10);

    // Define some 2D points.
    let points = vec![
        Point2D {
            x: 1.0,
            y: 2.0,
            data: Some("Point1"),
        },
        Point2D {
            x: 3.0,
            y: 4.0,
            data: Some("Point2"),
        },
        Point2D {
            x: 5.0,
            y: 6.0,
            data: Some("Point3"),
        },
    ];

    // Insert points into the Quadtree.
    tree.insert_bulk(&points);
}
```

### Python Examples

#### Quadtree (2D)

```python
from pyspart import Quadtree, Point2D

# Define the bounding area for the Quadtree.
boundary = {"x": 0.0, "y": 0.0, "width": 10.0, "height": 10.0}

# Create a new Quadtree with a maximum capacity of 3 points per node.
tree = Quadtree(boundary, 3)

# Define some 2D points.
point1 = Point2D(1.0, 2.0, "Point1")
point2 = Point2D(3.0, 4.0, "Point2")
point3 = Point2D(5.0, 6.0, "Point3")
point4 = Point2D(7.0, 8.0, "Point4")
point5 = Point2D(2.0, 3.0, "Point5")

# Insert points into the Quadtree.
tree.insert(point1)
tree.insert(point2)
tree.insert(point3)
tree.insert(point4)
tree.insert(point5)

# Perform a k-nearest neighbor (kNN) search.
neighbors = tree.knn_search(point1, 2)
print(f"kNN search results for {point1}: {neighbors}")

# Perform a range search with a radius of 5.0.
range_points = tree.range_search(point1, 5.0)
print(f"Range search results for {point1}: {range_points}")

# Remove a point from the tree.
tree.delete(point1)
```

#### Octree (3D)

```python
from pyspart import Octree, Point3D

# Define the bounding area for the Octree.
boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 10.0, "height": 10.0, "depth": 10.0}

# Create a new Octree with a maximum capacity of 3 points per node.
tree = Octree(boundary, 3)

# Define some 3D points.
point1 = Point3D(1.0, 2.0, 3.0, "Point1")
point2 = Point3D(3.0, 4.0, 5.0, "Point2")
point3 = Point3D(5.0, 6.0, 7.0, "Point3")
point4 = Point3D(7.0, 8.0, 9.0, "Point4")
point5 = Point3D(2.0, 3.0, 4.0, "Point5")

# Insert points into the Octree.
tree.insert(point1)
tree.insert(point2)
tree.insert(point3)
tree.insert(point4)
tree.insert(point5)

# Perform a kNN search.
neighbors = tree.knn_search(point1, 2)
print(f"kNN search results for {point1}: {neighbors}")

# Perform a range search with a radius of 5.0.
range_points = tree.range_search(point1, 5.0)
print(f"Range search results for {point1}: {range_points}")

# Remove a point from the tree.
tree.delete(point1)
```

#### Kd-tree (3D)

```python
from pyspart import KdTree3D, Point3D

# Create a new Kd-tree for 3D points.
tree = KdTree3D()

# Define some 3D points.
point1 = Point3D(1.0, 2.0, 3.0, "Point1")
point2 = Point3D(3.0, 4.0, 5.0, "Point2")
point3 = Point3D(5.0, 6.0, 7.0, "Point3")
point4 = Point3D(7.0, 8.0, 9.0, "Point4")
point5 = Point3D(2.0, 3.0, 4.0, "Point5")

# Insert points into the Kd-tree.
tree.insert(point1)
tree.insert(point2)
tree.insert(point3)
tree.insert(point4)
tree.insert(point5)

# Perform a kNN search.
neighbors = tree.knn_search(point1, 2)
print(f"kNN search results for {point1}: {neighbors}")

# Perform a range search with a radius of 5.0.
range_points = tree.range_search(point1, 5.0)
print(f"Range search results for {point1}: {range_points}")

# Remove a point from the tree.
tree.delete(point1)
```

#### R-tree (3D)

```python
from pyspart import RTree3D, Point3D

# Create a new R-tree with a maximum capacity of 4 points per node.
tree = RTree3D(4)

# Define some 3D points.
point1 = Point3D(1.0, 2.0, 3.0, "Point1")
point2 = Point3D(3.0, 4.0, 5.0, "Point2")
point3 = Point3D(5.0, 6.0, 7.0, "Point3")
point4 = Point3D(7.0, 8.0, 9.0, "Point4")
point5 = Point3D(2.0, 3.0, 4.0, "Point5")

# Insert points into the R-tree.
tree.insert(point1)
tree.insert(point2)
tree.insert(point3)
tree.insert(point4)
tree.insert(point5)

# Perform a kNN search.
neighbors = tree.knn_search(point1, 2)
print(f"kNN search results for {point1}: {neighbors}")

# Perform a range search with a radius of 5.0.
range_points = tree.range_search(point1, 5.0)
print(f"Range search results for {point1}: {range_points}")

# Remove a point from the tree.
tree.delete(point1)
```

#### R*-tree (3D)

```python
from pyspart import RStarTree3D, Point3D

# Create a new R*-tree with a maximum capacity of 4 points per node.
tree = RStarTree3D(4)

# Define some 3D points.
point1 = Point3D(1.0, 2.0, 3.0, "Point1")
point2 = Point3D(3.0, 4.0, 5.0, "Point2")
point3 = Point3D(5.0, 6.0, 7.0, "Point3")
point4 = Point3D(7.0, 8.0, 9.0, "Point4")
point5 = Point3D(2.0, 3.0, 4.0, "Point5")

# Insert points into the R*-tree.
tree.insert(point1)
tree.insert(point2)
tree.insert(point3)
tree.insert(point4)
tree.insert(point5)

# Perform a kNN search.
neighbors = tree.knn_search(point1, 2)
print(f"kNN search results for {point1}: {neighbors}")

# Perform a range search with a radius of 5.0.
range_points = tree.range_search(point1, 5.0)
print(f"Range search results for {point1}: {range_points}")

# Remove a point from the tree.
tree.delete(point1)
```

### Debugging Mode

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

### Serialization

Spart trees can be serialized and deserialized using the `serde` feature.

#### Rust

To enable serialization in Rust, you need to enable the `serde` feature in your `Cargo.toml` file:

```toml
[dependencies]
spart = { version = "0.1.0", features = ["serde"] }
```

Then, you can use `bincode` (or any other serde-compatible library) to serialize and deserialize the tree:

```rust
use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;

fn main() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut qt = Quadtree::new(&boundary, 4);
    qt.insert(Point2D::new(10.0, 20.0, Some("point1".to_string())));
    qt.insert(Point2D::new(50.0, 50.0, Some("point2".to_string())));

    // Serialize the tree
    let encoded: Vec<u8> = bincode::serialize(&qt).unwrap();

    // Deserialize the tree
    let decoded: Quadtree<String> = bincode::deserialize(&encoded[..]).unwrap();
}
```

#### Python

In Python, you can use the `save` and `load` methods to serialize and deserialize the tree to and from a file:

```python
from pyspart import Quadtree, Point2D

# Create a Quadtree and insert some points
boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
qt = Quadtree(boundary, 4)
qt.insert(Point2D(10.0, 20.0, "point1"))
qt.insert(Point2D(50.0, 50.0, "point2"))

# Save the tree to a file
qt.save("quadtree.spart")

# Load the tree from the file
loaded_qt = Quadtree.load("quadtree.spart")
```
