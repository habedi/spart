import pytest

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


def test_point2d_creation():
    p = Point2D(1.0, 2.0, {"some": "data"})
    assert p.x == 1.0
    assert p.y == 2.0
    assert p.data == {"some": "data"}


def test_point3d_creation():
    p = Point3D(1.0, 2.0, 3.0, {"some": "data"})
    assert p.x == 1.0
    assert p.y == 2.0
    assert p.z == 3.0
    assert p.data == {"some": "data"}


def test_quadtree():
    boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
    qt = Quadtree(boundary, 4)
    p1 = Point2D(10.0, 20.0, "p1")
    p2 = Point2D(50.0, 50.0, "p2")
    p3 = Point2D(90.0, 80.0, "p3")

    qt.insert(p1)
    qt.insert(p2)
    qt.insert(p3)

    # KNN Search
    results = qt.knn_search(Point2D(12.0, 22.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = qt.range_search(Point2D(50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert qt.delete(p2)
    results = qt.knn_search(Point2D(50.0, 50.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"


def test_octree():
    boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
    ot = Octree(boundary, 4)
    p1 = Point3D(10.0, 20.0, 30.0, "p1")
    p2 = Point3D(50.0, 50.0, 50.0, "p2")
    p3 = Point3D(90.0, 80.0, 70.0, "p3")

    ot.insert(p1)
    ot.insert(p2)
    ot.insert(p3)

    # KNN Search
    results = ot.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = ot.range_search(Point3D(50.0, 50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert ot.delete(p2)
    results = ot.knn_search(Point3D(50.0, 50.0, 50.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"


def test_kdtree2d():
    kd = KdTree2D()
    p1 = Point2D(1.0, 2.0, "p1")
    p2 = Point2D(5.0, 5.0, "p2")
    p3 = Point2D(9.0, 8.0, "p3")
    kd.insert(p1)
    kd.insert(p2)
    kd.insert(p3)

    # KNN Search
    results = kd.knn_search(Point2D(1.0, 2.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = kd.range_search(Point2D(5.0, 5.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert kd.delete(p2)
    results = kd.knn_search(Point2D(5.0, 5.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"


def test_kdtree3d():
    kd = KdTree3D()
    p1 = Point3D(1.0, 2.0, 3.0, "p1")
    p2 = Point3D(5.0, 5.0, 5.0, "p2")
    p3 = Point3D(9.0, 8.0, 7.0, "p3")
    kd.insert(p1)
    kd.insert(p2)
    kd.insert(p3)

    # KNN Search
    results = kd.knn_search(Point3D(1.0, 2.0, 3.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = kd.range_search(Point3D(5.0, 5.0, 5.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert kd.delete(p2)
    results = kd.knn_search(Point3D(5.0, 5.0, 5.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"


def test_rtree2d():
    rt = RTree2D(4)
    p1 = Point2D(10.0, 20.0, "p1")
    p2 = Point2D(50.0, 50.0, "p2")
    p3 = Point2D(90.0, 80.0, "p3")

    rt.insert(p1)
    rt.insert(p2)
    rt.insert(p3)

    # Range Search
    results = rt.range_search(Point2D(50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert rt.delete(p2)
    results = rt.range_search(Point2D(50.0, 50.0, None), 1.0)
    assert len(results) == 0


def test_rtree3d():
    rt = RTree3D(4)
    p1 = Point3D(10.0, 20.0, 30.0, "p1")
    p2 = Point3D(50.0, 50.0, 50.0, "p2")
    p3 = Point3D(90.0, 80.0, 70.0, "p3")

    rt.insert(p1)
    rt.insert(p2)
    rt.insert(p3)

    # Range Search
    results = rt.range_search(Point3D(50.0, 50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert rt.delete(p2)
    results = rt.range_search(Point3D(50.0, 50.0, 50.0, None), 1.0)
    assert len(results) == 0


def test_rstartree2d():
    rst = RStarTree2D(4)
    p1 = Point2D(10.0, 20.0, "p1")
    p2 = Point2D(50.0, 50.0, "p2")
    p3 = Point2D(90.0, 80.0, "p3")

    rst.insert(p1)
    rst.insert(p2)
    rst.insert(p3)

    # KNN Search
    results = rst.knn_search(Point2D(12.0, 22.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = rst.range_search(Point2D(50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert rst.delete(p2)
    results = rst.knn_search(Point2D(50.0, 50.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"


def test_rstartree3d():
    rst = RStarTree3D(4)
    p1 = Point3D(10.0, 20.0, 30.0, "p1")
    p2 = Point3D(50.0, 50.0, 50.0, "p2")
    p3 = Point3D(90.0, 80.0, 70.0, "p3")

    rst.insert(p1)
    rst.insert(p2)
    rst.insert(p3)

    # KNN Search
    results = rst.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"

    # Range Search
    results = rst.range_search(Point3D(50.0, 50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"

    # Deletion
    assert rst.delete(p2)
    results = rst.knn_search(Point3D(50.0, 50.0, 50.0, None), 1)
    assert len(results) == 1
    assert results[0].data != "p2"

def test_empty_trees():
    # Quadtree
    boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
    qt = Quadtree(boundary, 4)
    assert not qt.knn_search(Point2D(0, 0, None), 1)
    assert not qt.range_search(Point2D(0, 0, None), 1)
    assert not qt.delete(Point2D(0, 0, None))

    # Octree
    boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
    ot = Octree(boundary, 4)
    assert not ot.knn_search(Point3D(0, 0, 0, None), 1)
    assert not ot.range_search(Point3D(0, 0, 0, None), 1)
    assert not ot.delete(Point3D(0, 0, 0, None))

    # KdTree2D
    kd2 = KdTree2D()
    assert not kd2.knn_search(Point2D(0, 0, None), 1)
    assert not kd2.range_search(Point2D(0, 0, None), 1)
    assert not kd2.delete(Point2D(0, 0, None))

    # KdTree3D
    kd3 = KdTree3D()
    assert not kd3.knn_search(Point3D(0, 0, 0, None), 1)
    assert not kd3.range_search(Point3D(0, 0, 0, None), 1)
    assert not kd3.delete(Point3D(0, 0, 0, None))

    # RTree2D
    rt2 = RTree2D(4)
    assert not rt2.knn_search(Point2D(0, 0, None), 1)
    assert not rt2.range_search(Point2D(0, 0, None), 1)
    assert not rt2.delete(Point2D(0, 0, None))

    # RTree3D
    rt3 = RTree3D(4)
    assert not rt3.knn_search(Point3D(0, 0, 0, None), 1)
    assert not rt3.range_search(Point3D(0, 0, 0, None), 1)
    assert not rt3.delete(Point3D(0, 0, 0, None))

    # RStarTree2D
    rst2 = RStarTree2D(4)
    assert not rst2.knn_search(Point2D(0, 0, None), 1)
    assert not rst2.range_search(Point2D(0, 0, None), 1)
    assert not rst2.delete(Point2D(0, 0, None))

    # RStarTree3D
    rst3 = RStarTree3D(4)
    assert not rst3.knn_search(Point3D(0, 0, 0, None), 1)
    assert not rst3.range_search(Point3D(0, 0, 0, None), 1)
    assert not rst3.delete(Point3D(0, 0, 0, None))

def test_knn_edge_cases():
    kd = KdTree2D()
    points = [Point2D(1, 2, 'p1'), Point2D(3, 4, 'p2')]
    kd.insert_bulk(points)

    # k=0
    assert not kd.knn_search(Point2D(0, 0, None), 0)

    # k > num_points
    assert len(kd.knn_search(Point2D(0, 0, None), 5)) == 2

def test_range_zero_radius():
    kd = KdTree2D()
    p1 = Point2D(1, 2, 'p1')
    kd.insert(p1)

    results = kd.range_search(Point2D(1, 2, None), 0)
    assert len(results) == 1
    assert results[0].data == 'p1'

def test_duplicates():
    kd = KdTree2D()
    p1 = Point2D(1, 2, 'p1')
    p2 = Point2D(1, 2, 'p1')
    kd.insert(p1)
    kd.insert(p2)

    assert len(kd.knn_search(Point2D(1, 2, None), 2)) == 2

    assert kd.delete(p1)
    assert len(kd.knn_search(Point2D(1, 2, None), 1)) == 1
