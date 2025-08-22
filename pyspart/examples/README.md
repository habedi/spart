## PySpart Examples

| # | File                           | Description                                                                                              |
|---|--------------------------------|----------------------------------------------------------------------------------------------------------|
| 1 | [kdtree.py](kdtree.py)         | Shows the usage of a 2D and 3D KdTree for inserting points and performing a kNN search                   |
| 2 | [octree.py](octree.py)         | Shows the usage of an Octree for inserting 3D points in a defined boundary and performing a kNN search   |
| 3 | [quadtree.py](quadtree.py)     | Shows the usage of a Quadtree for inserting 2D points in a defined boundary and performing a kNN search  |
| 4 | [rtree.py](rtree.py)           | Shows the usage of a 2D and 3D R-tree for inserting points and performing a range search                 |
| 5 | [rstar_tree.py](rstar_tree.py) | Shows the usage of a 2D and 3D R*-tree for inserting points and performing a kNN search and range search |

### How to run

- Make sure you have a virtual environment and PySpart built for development.
- From the repository root, you can run all Python examples at once:
    - `make run-py-examples`
- Or run a specific example (after setting up the `venv` and running `make develop-py`):
    - `python pyspart/examples/r_star_tree.py`
