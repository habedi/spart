# Spart Documentation

## Space Partitioning Trees

Currently, the following trees are implemented: Quadtree, Octree, Kd-tree, R-tree, and BSP-tree indexing and querying
2D and 3D point data.

### General Interface

#### Point

A point is a tuple of coordinates, x, y, and possibly z plus an optional data payload.

#### Tree

Each tree implements the following methods:

- `new` - creates a new tree.
- `insert` - inserts a point into the tree.
- `delete` - removes a point from the tree.
- `knn_search` - finds the k nearest neighbors to a query point.
- `range_search` - finds all points within a given range of a query point.




