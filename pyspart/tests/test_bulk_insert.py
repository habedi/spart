import pytest

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


def test_quadtree_bulk():
    boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
    qt = Quadtree(boundary, 4)
    points = [
        Point2D(10.0, 20.0, "p1"),
        Point2D(50.0, 50.0, "p2"),
        Point2D(90.0, 80.0, "p3"),
    ]
    qt.insert_bulk(points)

    # KNN Search
    results = qt.knn_search(Point2D(12.0, 22.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"


def test_octree_bulk():
    boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
    ot = Octree(boundary, 4)
    points = [
        Point3D(10.0, 20.0, 30.0, "p1"),
        Point3D(50.0, 50.0, 50.0, "p2"),
        Point3D(90.0, 80.0, 70.0, "p3"),
    ]
    ot.insert_bulk(points)

    # KNN Search
    results = ot.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"


def test_kdtree2d_bulk():
    kd = KdTree2D()
    points = [
        Point2D(1.0, 2.0, "p1"),
        Point2D(5.0, 5.0, "p2"),
        Point2D(9.0, 8.0, "p3"),
    ]
    kd.insert_bulk(points)

    # KNN Search
    results = kd.knn_search(Point2D(1.0, 2.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"


def test_kdtree3d_bulk():
    kd = KdTree3D()
    points = [
        Point3D(1.0, 2.0, 3.0, "p1"),
        Point3D(5.0, 5.0, 5.0, "p2"),
        Point3D(9.0, 8.0, 7.0, "p3"),
    ]
    kd.insert_bulk(points)

    # KNN Search
    results = kd.knn_search(Point3D(1.0, 2.0, 3.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"


def test_rtree2d_bulk():
    rt = RTree2D(4)
    points = [
        Point2D(10.0, 20.0, "p1"),
        Point2D(50.0, 50.0, "p2"),
        Point2D(90.0, 80.0, "p3"),
    ]
    rt.insert_bulk(points)

    # Range Search
    results = rt.range_search(Point2D(50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"


def test_rtree3d_bulk():
    rt = RTree3D(4)
    points = [
        Point3D(10.0, 20.0, 30.0, "p1"),
        Point3D(50.0, 50.0, 50.0, "p2"),
        Point3D(90.0, 80.0, 70.0, "p3"),
    ]
    rt.insert_bulk(points)

    # Range Search
    results = rt.range_search(Point3D(50.0, 50.0, 50.0, None), 1.0)
    assert len(results) == 1
    assert results[0].data == "p2"


def test_rstartree2d_bulk():
    rst = RStarTree2D(4)
    points = [
        Point2D(10.0, 20.0, "p1"),
        Point2D(50.0, 50.0, "p2"),
        Point2D(90.0, 80.0, "p3"),
    ]
    rst.insert_bulk(points)

    # KNN Search
    results = rst.knn_search(Point2D(12.0, 22.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"


def test_rstartree3d_bulk():
    rst = RStarTree3D(4)
    points = [
        Point3D(10.0, 20.0, 30.0, "p1"),
        Point3D(50.0, 50.0, 50.0, "p2"),
        Point3D(90.0, 80.0, 70.0, "p3"),
    ]
    rst.insert_bulk(points)

    # KNN Search
    results = rst.knn_search(Point3D(12.0, 22.0, 32.0, None), 1)
    assert len(results) == 1
    assert results[0].data == "p1"
