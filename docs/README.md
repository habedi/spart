## Spart Documentation

Spart provides a set of spatial data structures for indexing and querying point data.
The basic building blocks of Spart are **point** and **tree**.

### Point

A point is a tuple of coordinates plus an optional data payload of any type.
There are two types of points: `Point2D` and `Point3D`.

Example of 2D and 3D points:

```rust
use spart::geometry::{Point2D, Point3D};

let point_2d = Point2D {
x:1.0, y:2.0,
data: Some("A 2D Point")
};

let point_3d = Point3D {
x:1.0, y:2.0, z:3.0,
data: Some("A 3D Point")
};
```

> [!NOTE]
> For BSP-trees, we need to use Point2DBSP and Point3DBSP instead of Point2D and Point3D.
> Point2DBSP and Point3DBSP wrap the point with additional properties required for BSP-trees.

### Tree

A tree is a spatial data structure that indexes points and provides methods for querying them.

Currently, the following trees are implemented:
- Quadtree (2D)
- Octree (3D)
- Kd-tree (2D and 3D)
- R-tree (2D and 3D)
- BSP-tree (2D and 3D)

A tree provides at least the following methods:

- `new`: creates a new tree given the following parameters:
    - The bounding area of the tree (for Quadtree and Octree only)
    - The number of dimensions (for Kd-tree only)
    - The maximum capacity of points per node (for Quadtree, Octree, R-tree, and BSP-tree)
- `insert`: inserts a point into the tree.
- `delete`: removes a point from the tree.
- `knn_search`: finds the k nearest neighbors to a query point.
    - The inputs are the query point and the number of neighbors to find.
- `range_search`: finds all points within a given range of a query point.
    - The inputs are the query point and the range within which to search.

> [!NOTE]
> Currently, the following properties hold for all trees:
> - Duplicates are allowed: inserting a duplicate point will add another copy to the tree.
> - Searches return duplicates: both knn_search and range_search can return duplicate points if they were previously inserted.
> - Deletion removes duplicates: the delete operation removes all instances of the point from the tree.
> 
> The distance metric used for nearest neighbor and range searches is the Euclidean distance.

### Examples

Below are some examples of how to use the different trees in Spart.
Check out the [tests](../tests/) directory for more detailed examples.

#### Quadtree (2D)

```rust
use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;

// Define the bounding area for the Quadtree
let boundary = Rectangle {
x: 0.0, y: 0.0,
width: 10.0, height: 10.0,
};

// Create a new Quadtree with a maximum capacity of 3 points per node
let mut tree = Quadtree::new(&boundary, 3);

let point1 = Point2D {
x: 1.0, y: 2.0,
data: Some("Point1"),
};

let point2 = Point2D {
x: 3.0, y: 4.0,
data: Some("Point2"),
};

let point3 = Point2D {
x: 5.0, y: 6.0,
data: Some("Point3"),
};

let point4 = Point2D {
x: 7.0, y: 8.0,
data: Some("Point4"),
};

let point5 = Point2D {
x: 2.0, y: 3.0,
data: Some("Point5"),
};

// Insert points into the Quadtree
tree.insert(point1.clone());
tree.insert(point2.clone());
tree.insert(point3.clone());
tree.insert(point4.clone());
tree.insert(point5.clone());

let neighbors = tree.knn_search(&point1, 2);
println!("kNN search results for {:?}: {:?}", point1, neighbors);

let range_points = tree.range_search(&point1, 5.0);
println!("Range search results for {:?}: {:?}", point1, range_points);

// Remove a point from the tree
tree.delete(&point1);
```

#### Octree (3D)

```rust
use spart::geometry::{Cube, Point3D};
use spart::octree::Octree;

// Define the bounding area for the Octree
let boundary = Cube {
x: 0.0, y: 0.0, z: 0.0,
width: 10.0, height: 10.0, depth: 10.0,
};

// Create a new Octree with a maximum capacity of 3 points per node
let mut tree = Octree::new(&boundary, 3);

let point1 = Point3D {
x: 1.0, y: 2.0, z: 3.0,
data: Some("Point1"),
};

let point2 = Point3D {
x: 3.0, y: 4.0, z: 5.0,
data: Some("Point2"),
};

let point3 = Point3D {
x: 5.0, y: 6.0, z: 7.0,
data: Some("Point3"),
};

let point4 = Point3D {
x: 7.0, y: 8.0, z: 9.0,
data: Some("Point4"),
};

let point5 = Point3D {
x: 2.0, y: 3.0, z: 4.0,
data: Some("Point5"),
};

// Insert points into the Octree
tree.insert(point1.clone());
tree.insert(point2.clone());
tree.insert(point3.clone());
tree.insert(point4.clone());
tree.insert(point5.clone());

let neighbors = tree.knn_search(&point1, 2);
println!("kNN search results for {:?}: {:?}", point1, neighbors);

let range_points = tree.range_search(&point1, 5.0);
println!("Range search results for {:?}: {:?}", point1, range_points);

// Remove a point from the tree
tree.delete(&point1);
```

#### Kd-tree (3D)

```rust
use spart::geometry::Point3D;
use spart::kd_tree::KdTree;

// Create a new Kd-tree for 3D points
let mut tree = KdTree::new(3);

let point1 = Point3D {
x: 1.0, y: 2.0, z: 3.0,
data: Some("Point1"),
};

let point2 = Point3D {
x: 3.0, y: 4.0, z: 5.0,
data: Some("Point2"),
};

let point3 = Point3D {
x: 5.0, y: 6.0, z: 7.0,
data: Some("Point3"),
};

let point4 = Point3D {
x: 7.0, y: 8.0, z: 9.0,
data: Some("Point4"),
};

let point5 = Point3D {
x: 2.0, y: 3.0, z: 4.0,
data: Some("Point5"),
};

// Insert points into the Kd-tree
tree.insert(point1.clone());
tree.insert(point2.clone());
tree.insert(point3.clone());
tree.insert(point4.clone());
tree.insert(point5.clone());

let neighbors = tree.knn_search(&point1, 2);
println!("kNN search results for {:?}: {:?}", point1, neighbors);

let range_points = tree.range_search(&point1, 5.0);
println!("Range search results for {:?}: {:?}", point1, range_points);

// Remove a point from the tree
tree.delete(&point1);
```

#### R-tree (3D)

```rust
use spart::geometry::Point3D;
use spart::r_tree::RTree;

// Create a new R-tree with a maximum capacity of 4 points per node
let mut tree = RTree::new(4);

let point1 = Point3D {
    x: 1.0, y: 2.0, z: 3.0,
    data: Some("Point1"),
};

let point2 = Point3D {
    x: 3.0, y: 4.0, z: 5.0,
    data: Some("Point2"),
};

let point3 = Point3D {
    x: 5.0, y: 6.0, z: 7.0,
    data: Some("Point3"),
};

let point4 = Point3D {
    x: 7.0, y: 8.0, z: 9.0,
    data: Some("Point4"),
};

let point5 = Point3D {
    x: 2.0, y: 3.0, z: 4.0,
    data: Some("Point5"),
};

// Insert points into the R-tree
tree.insert(point1.clone());
tree.insert(point2.clone());
tree.insert(point3.clone());
tree.insert(point4.clone());
tree.insert(point5.clone());

let neighbors = tree.knn_search(&point1, 2);
println!("kNN search results for {:?}: {:?}", point1, neighbors);

let range_points = tree.range_search(&point1, 5.0);
println!("Range search results for {:?}: {:?}", point1, range_points);

// Remove a point from the tree
tree.delete(&point1);
```

#### BSP-tree (3D)

```rust
use spart::bsp_tree::{BSPTree, Point3DBSP};
use spart::geometry::Point3D;

// Create a new BSP-tree with a capacity of 3 points per node
let mut tree: BSPTree<Point3DBSP<&str>> = BSPTree::new(3);

let point1 = Point3DBSP {
    point: Point3D {
        x: 1.0, y: 2.0, z: 3.0,
        data: Some("Wrapped Point1"),
}};

let point2 = Point3DBSP {
    point: Point3D {
        x: 3.0, y: 4.0, z: 5.0,
        data: Some("Wrapped Point2"),
}};

let point3 = Point3DBSP {
    point: Point3D {
        x: 5.0, y: 6.0, z: 7.0,
        data: Some("Wrapped Point3"),
}};

let point4 = Point3DBSP {
    point: Point3D {
        x: 7.0, y: 8.0, z: 9.0,
        data: Some("Wrapped Point4"),
}};

let point5 = Point3DBSP {
    point: Point3D {
        x: 2.0, y: 3.0, z: 4.0,
        data: Some("Wrapped Point5"),
}};

// Insert points into the tree
tree.insert(point1.clone());
tree.insert(point2.clone());
tree.insert(point3.clone());
tree.insert(point4.clone());
tree.insert(point5.clone());

let neighbors = tree.knn_search(&point1, 2);
println!("kNN search results for {:?}: {:?}", point1, neighbors);

let range_points = tree.range_search(&point1, 5.0);
println!("Range search results for {:?}: {:?}", point1, range_points);

// Remove a point from the tree
tree.delete(&point1);
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
> When debugging mode is enabled, Spart will be very verbose and print a lot of debug information to the console.
> So, it is recommended to use this only for debugging purposes.
