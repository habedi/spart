## PySpart

[<img alt="license" src="https://img.shields.io/badge/license-MIT-007ec6?style=flat&logo=open-source-initiative" height="20">](https://github.com/habedi/spart/tree/main/pyspart/LICENSE)
[<img alt="python version" src="https://img.shields.io/badge/Python-%3E=3.10-blue?style=flat&logo=python" height="20">](https://github.com/habedi/spart/tree/main/pyspart)
[<img alt="pypi" src="https://img.shields.io/pypi/v/pyspart?style=flat&logo=pypi&color=fc8d62" height="20">](https://pypi.org/project/pyspart)
[<img alt="downloads" src="https://img.shields.io/pypi/dm/pyspart?style=flat&logo=pypi" height="20">](https://pypi.org/project/pyspart)

Python bindings for the [Spart](https://github.com/habedi/spart) library.

### Installation

```bash
pip install pyspart
````

### Examples

Below are some examples of how to use the different trees in PySpart.

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

Check out the [examples](https://github.com/habedi/spart/tree/main/pyspart/examples) directory for more examples.

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

### License

PySpart is licensed under the [MIT License](https://github.com/habedi/spart/tree/main/pyspart/LICENSE).
