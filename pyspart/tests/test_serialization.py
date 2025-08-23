import os
import pytest

from pyspart import (
    Quadtree,
    Octree,
    KdTree2D,
    KdTree3D,
    RTree2D,
    RTree3D,
    RStarTree2D,
    RStarTree3D,
    Point2D,
    Point3D,
)


@pytest.fixture
def temp_path(request):
    """A pytest fixture to create a temporary file path and clean it up after the test."""
    path = f"test_{request.node.name}.spart"
    yield path
    if os.path.exists(path):
        os.remove(path)


def test_quadtree_serialization(temp_path):
    boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
    qt = Quadtree(boundary, 4)
    qt.insert(Point2D(10.0, 20.0, {"data": "point1"}))
    qt.insert(Point2D(50.0, 50.0, {"data": "point2"}))

    qt.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_qt = Quadtree.load(temp_path)

    original_neighbors = qt.knn_search(Point2D(12.0, 22.0, None), 1)
    loaded_neighbors = loaded_qt.knn_search(Point2D(12.0, 22.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].data == loaded_neighbors[0].data


def test_octree_serialization(temp_path):
    boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
    octree = Octree(boundary, 4)
    octree.insert(Point3D(10.0, 20.0, 30.0, {"data": "point1"}))
    octree.insert(Point3D(50.0, 50.0, 50.0, {"data": "point2"}))

    octree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_octree = Octree.load(temp_path)

    original_neighbors = octree.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    loaded_neighbors = loaded_octree.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].z == loaded_neighbors[0].z
    assert original_neighbors[0].data == loaded_neighbors[0].data


def test_kdtree2d_serialization(temp_path):
    tree = KdTree2D()
    tree.insert(Point2D(1.0, 2.0, {"data": "point1"}))
    tree.insert(Point2D(3.0, 4.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = KdTree2D.load(temp_path)

    original_neighbors = tree.knn_search(Point2D(2.0, 3.0, None), 1)
    loaded_neighbors = loaded_tree.knn_search(Point2D(2.0, 3.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].data == loaded_neighbors[0].data


def test_kdtree3d_serialization(temp_path):
    tree = KdTree3D()
    tree.insert(Point3D(1.0, 2.0, 3.0, {"data": "point1"}))
    tree.insert(Point3D(4.0, 5.0, 6.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = KdTree3D.load(temp_path)

    original_neighbors = tree.knn_search(Point3D(2.0, 3.0, 4.0, None), 1)
    loaded_neighbors = loaded_tree.knn_search(Point3D(2.0, 3.0, 4.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].z == loaded_neighbors[0].z
    assert original_neighbors[0].data == loaded_neighbors[0].data


def test_rtree2d_serialization(temp_path):
    tree = RTree2D(4)
    tree.insert(Point2D(10.0, 20.0, {"data": "point1"}))
    tree.insert(Point2D(50.0, 50.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = RTree2D.load(temp_path)

    original_results = tree.range_search(Point2D(12.0, 22.0, None), 10.0)
    loaded_results = loaded_tree.range_search(Point2D(12.0, 22.0, None), 10.0)

    assert len(original_results) == len(loaded_results)
    assert original_results[0].x == loaded_results[0].x
    assert original_results[0].y == loaded_results[0].y
    assert original_results[0].data == loaded_results[0].data


def test_rtree3d_serialization(temp_path):
    tree = RTree3D(4)
    tree.insert(Point3D(10.0, 20.0, 30.0, {"data": "point1"}))
    tree.insert(Point3D(50.0, 50.0, 50.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = RTree3D.load(temp_path)

    original_results = tree.range_search(Point3D(12.0, 22.0, 32.0, None), 10.0)
    loaded_results = loaded_tree.range_search(Point3D(12.0, 22.0, 32.0, None), 10.0)

    assert len(original_results) == len(loaded_results)
    assert original_results[0].x == loaded_results[0].x
    assert original_results[0].y == loaded_results[0].y
    assert original_results[0].z == loaded_results[0].z
    assert original_results[0].data == loaded_results[0].data


def test_rstartree2d_serialization(temp_path):
    tree = RStarTree2D(4)
    tree.insert(Point2D(10.0, 20.0, {"data": "point1"}))
    tree.insert(Point2D(50.0, 50.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = RStarTree2D.load(temp_path)

    original_neighbors = tree.knn_search(Point2D(12.0, 22.0, None), 1)
    loaded_neighbors = loaded_tree.knn_search(Point2D(12.0, 22.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].data == loaded_neighbors[0].data


def test_rstartree3d_serialization(temp_path):
    tree = RStarTree3D(4)
    tree.insert(Point3D(10.0, 20.0, 30.0, {"data": "point1"}))
    tree.insert(Point3D(50.0, 50.0, 50.0, {"data": "point2"}))

    tree.save(temp_path)
    assert os.path.exists(temp_path)

    loaded_tree = RStarTree3D.load(temp_path)

    original_neighbors = tree.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    loaded_neighbors = loaded_tree.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)

    assert len(original_neighbors) == len(loaded_neighbors)
    assert original_neighbors[0].x == loaded_neighbors[0].x
    assert original_neighbors[0].y == loaded_neighbors[0].y
    assert original_neighbors[0].z == loaded_neighbors[0].z
    assert original_neighbors[0].data == loaded_neighbors[0].data
