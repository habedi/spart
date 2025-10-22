"""
Integration tests for serialization/deserialization with PyData.

Tests that Python objects can be properly serialized and deserialized
through Rust's bincode serialization.
"""
import os
import pytest
import tempfile

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


class TestQuadtreeSerialization:
    """Test serialization for Quadtree."""

    def test_save_and_load_quadtree(self):
        """Test that Quadtree can be saved and loaded correctly."""
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        tree = Quadtree(boundary, 4)

        # Insert points with various data types
        points = [
            Point2D(10.0, 20.0, {"id": 1, "name": "first"}),
            Point2D(30.0, 40.0, {"id": 2, "name": "second"}),
            Point2D(50.0, 60.0, [1, 2, 3, 4, 5]),
            Point2D(70.0, 80.0, "simple string"),
        ]

        for p in points:
            tree.insert(p)

        # Save to file
        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)

            # Load from file
            loaded_tree = Quadtree.load(filepath)

            # Verify all points are present
            for original in points:
                results = loaded_tree.knn_search(
                    Point2D(original.x, original.y, None), 1
                )
                assert len(results) == 1
                retrieved = results[0]
                assert retrieved.x == original.x
                assert retrieved.y == original.y
                assert retrieved.data == original.data
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)

    def test_save_empty_quadtree(self):
        """Test serializing an empty Quadtree."""
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        tree = Quadtree(boundary, 4)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = Quadtree.load(filepath)

            # Should be empty
            results = loaded_tree.range_search(Point2D(50.0, 50.0, None), 100.0)
            assert len(results) == 0
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


class TestOctreeSerialization:
    """Test serialization for Octree."""

    def test_save_and_load_octree(self):
        """Test that Octree can be saved and loaded correctly."""
        boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
        tree = Octree(boundary, 4)

        points = [
            Point3D(10.0, 20.0, 30.0, {"type": "A"}),
            Point3D(40.0, 50.0, 60.0, {"type": "B"}),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = Octree.load(filepath)

            for original in points:
                results = loaded_tree.knn_search(
                    Point3D(original.x, original.y, original.z, None), 1
                )
                assert len(results) == 1
                assert results[0].data == original.data
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


class TestKdTreeSerialization:
    """Test serialization for KdTree."""

    def test_save_and_load_kdtree2d(self):
        """Test that KdTree2D can be saved and loaded correctly."""
        tree = KdTree2D()

        points = [
            Point2D(10.0, 20.0, "A"),
            Point2D(30.0, 40.0, "B"),
            Point2D(50.0, 60.0, "C"),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = KdTree2D.load(filepath)

            # Verify by searching
            results = loaded_tree.knn_search(Point2D(10.0, 20.0, None), 1)
            assert len(results) == 1
            assert results[0].data == "A"
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)

    def test_save_and_load_kdtree3d(self):
        """Test that KdTree3D can be saved and loaded correctly."""
        tree = KdTree3D()

        points = [
            Point3D(10.0, 20.0, 30.0, {"id": 1}),
            Point3D(40.0, 50.0, 60.0, {"id": 2}),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = KdTree3D.load(filepath)

            results = loaded_tree.knn_search(Point3D(10.0, 20.0, 30.0, None), 1)
            assert len(results) == 1
            assert results[0].data["id"] == 1
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


class TestRTreeSerialization:
    """Test serialization for RTree."""

    def test_save_and_load_rtree2d(self):
        """Test that RTree2D can be saved and loaded correctly."""
        tree = RTree2D(4)

        points = [
            Point2D(10.0, 20.0, [1, 2, 3]),
            Point2D(30.0, 40.0, [4, 5, 6]),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = RTree2D.load(filepath)

            results = loaded_tree.knn_search(Point2D(10.0, 20.0, None), 1)
            assert len(results) == 1
            assert results[0].data == [1, 2, 3]
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)

    def test_save_and_load_rtree3d(self):
        """Test that RTree3D can be saved and loaded correctly."""
        tree = RTree3D(4)

        points = [
            Point3D(10.0, 20.0, 30.0, "point1"),
            Point3D(40.0, 50.0, 60.0, "point2"),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = RTree3D.load(filepath)

            results = loaded_tree.knn_search(Point3D(10.0, 20.0, 30.0, None), 1)
            assert len(results) == 1
            assert results[0].data == "point1"
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


class TestRStarTreeSerialization:
    """Test serialization for RStarTree."""

    def test_save_and_load_rstartree2d(self):
        """Test that RStarTree2D can be saved and loaded correctly."""
        tree = RStarTree2D(4)

        complex_data = {"nested": {"dict": True}, "list": [1, 2, 3]}
        points = [
            Point2D(10.0, 20.0, complex_data),
            Point2D(30.0, 40.0, "simple"),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = RStarTree2D.load(filepath)

            results = loaded_tree.knn_search(Point2D(10.0, 20.0, None), 1)
            assert len(results) == 1
            assert results[0].data == complex_data
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)

    def test_save_and_load_rstartree3d(self):
        """Test that RStarTree3D can be saved and loaded correctly."""
        tree = RStarTree3D(4)

        points = [
            Point3D(10.0, 20.0, 30.0, {"x": 1}),
            Point3D(40.0, 50.0, 60.0, {"x": 2}),
        ]

        for p in points:
            tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded_tree = RStarTree3D.load(filepath)

            results = loaded_tree.knn_search(Point3D(10.0, 20.0, 30.0, None), 1)
            assert len(results) == 1
            assert results[0].data["x"] == 1
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


class TestSerializationWithComplexPyData:
    """Test serialization with various complex Python data types."""

    def test_serialize_with_tuple_data(self):
        """Test serialization with tuple data."""
        tree = KdTree2D()
        p = Point2D(10.0, 20.0, (1, 2, 3))
        tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded = KdTree2D.load(filepath)
            results = loaded.knn_search(Point2D(10.0, 20.0, None), 1)
            assert results[0].data == (1, 2, 3)
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)

    def test_serialize_with_nested_structures(self):
        """Test serialization with deeply nested data structures."""
        tree = RTree2D(4)
        nested = {
            "level1": {
                "level2": {
                    "level3": ["a", "b", "c"]
                }
            },
            "numbers": [1, 2, 3, 4, 5]
        }
        p = Point2D(10.0, 20.0, nested)
        tree.insert(p)

        with tempfile.NamedTemporaryFile(delete=False, suffix=".bin") as f:
            filepath = f.name

        try:
            tree.save(filepath)
            loaded = RTree2D.load(filepath)
            results = loaded.knn_search(Point2D(10.0, 20.0, None), 1)
            assert results[0].data == nested
        finally:
            if os.path.exists(filepath):
                os.unlink(filepath)


"""
Unit tests for edge cases in Python bindings point conversions.
"""
import pytest
from pyspart import Point2D, Point3D


class TestPoint2DEdgeCases:
    """Test edge cases for Point2D."""

    def test_point2d_with_none_data(self):
        """Test creating a Point2D with None as data."""
        p = Point2D(1.0, 2.0, None)
        assert p.x == 1.0
        assert p.y == 2.0
        assert p.data is None

    def test_point2d_with_complex_data(self):
        """Test Point2D with complex nested data structures."""
        complex_data = {
            "nested": {"dict": [1, 2, 3]},
            "list": ["a", "b", "c"],
            "number": 42.5
        }
        p = Point2D(1.0, 2.0, complex_data)
        assert p.x == 1.0
        assert p.y == 2.0
        assert p.data == complex_data

    def test_point2d_with_custom_object(self):
        """Test Point2D with custom Python object."""

        class CustomObject:
            def __init__(self, value):
                self.value = value

            def __eq__(self, other):
                return isinstance(other, CustomObject) and self.value == other.value

        obj = CustomObject(123)
        p = Point2D(1.0, 2.0, obj)
        assert p.x == 1.0
        assert p.y == 2.0
        assert p.data.value == 123

    def test_point2d_equality(self):
        """Test Point2D equality comparison."""
        p1 = Point2D(1.0, 2.0, "data1")
        p2 = Point2D(1.0, 2.0, "data1")
        p3 = Point2D(1.0, 2.0, "data2")
        p4 = Point2D(1.0, 3.0, "data1")

        # Same coordinates and data should be equal
        assert p1 == p2
        # Different data should not be equal
        assert p1 != p3
        # Different coordinates should not be equal
        assert p1 != p4

    def test_point2d_with_extreme_coordinates(self):
        """Test Point2D with extreme coordinate values."""
        import math

        # Very large values
        p1 = Point2D(1e308, 1e308, "large")
        assert p1.x == 1e308
        assert p1.y == 1e308

        # Very small values
        p2 = Point2D(1e-308, 1e-308, "small")
        assert p2.x == 1e-308
        assert p2.y == 1e-308

        # Negative values
        p3 = Point2D(-1e6, -1e6, "negative")
        assert p3.x == -1e6
        assert p3.y == -1e6


class TestPoint3DEdgeCases:
    """Test edge cases for Point3D."""

    def test_point3d_with_none_data(self):
        """Test creating a Point3D with None as data."""
        p = Point3D(1.0, 2.0, 3.0, None)
        assert p.x == 1.0
        assert p.y == 2.0
        assert p.z == 3.0
        assert p.data is None

    def test_point3d_with_complex_data(self):
        """Test Point3D with complex nested data structures."""
        complex_data = {"key": [1, 2, {"nested": True}]}
        p = Point3D(1.0, 2.0, 3.0, complex_data)
        assert p.data == complex_data

    def test_point3d_equality(self):
        """Test Point3D equality comparison."""
        p1 = Point3D(1.0, 2.0, 3.0, "data1")
        p2 = Point3D(1.0, 2.0, 3.0, "data1")
        p3 = Point3D(1.0, 2.0, 3.0, "data2")
        p4 = Point3D(1.0, 2.0, 4.0, "data1")

        assert p1 == p2
        assert p1 != p3
        assert p1 != p4

    def test_point3d_with_extreme_coordinates(self):
        """Test Point3D with extreme coordinate values."""
        p = Point3D(1e308, -1e308, 0.0, "extreme")
        assert p.x == 1e308
        assert p.y == -1e308
        assert p.z == 0.0


class TestPointConversionEdgeCases:
    """Test edge cases in point conversions between Python and Rust."""

    def test_roundtrip_point2d(self):
        """Test that Point2D survives a roundtrip through tree operations."""
        from pyspart import Quadtree

        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        tree = Quadtree(boundary, 4)

        # Create point with complex data
        original_data = {"id": 1, "name": "test", "values": [1, 2, 3]}
        p = Point2D(10.0, 20.0, original_data)

        # Insert and retrieve
        tree.insert(p)
        results = tree.knn_search(Point2D(10.0, 20.0, None), 1)

        assert len(results) == 1
        retrieved = results[0]
        assert retrieved.x == 10.0
        assert retrieved.y == 20.0
        assert retrieved.data == original_data

    def test_roundtrip_point3d(self):
        """Test that Point3D survives a roundtrip through tree operations."""
        from pyspart import Octree

        boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
        tree = Octree(boundary, 4)

        # Create point with complex data
        original_data = {"id": 1, "coords": [10, 20, 30]}
        p = Point3D(10.0, 20.0, 30.0, original_data)

        # Insert and retrieve
        tree.insert(p)
        results = tree.knn_search(Point3D(10.0, 20.0, 30.0, None), 1)

        assert len(results) == 1
        retrieved = results[0]
        assert retrieved.x == 10.0
        assert retrieved.y == 20.0
        assert retrieved.z == 30.0
        assert retrieved.data == original_data

    def test_multiple_tree_types_consistency(self):
        """Test that different tree types handle points consistently."""
        from pyspart import Quadtree, KdTree2D, RTree2D, RStarTree2D

        # Create points
        points = [
            Point2D(10.0, 20.0, "p1"),
            Point2D(30.0, 40.0, "p2"),
            Point2D(50.0, 60.0, "p3"),
        ]

        # Test Quadtree
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        qt = Quadtree(boundary, 4)
        for p in points:
            qt.insert(p)

        # Test KdTree
        kd = KdTree2D()
        for p in points:
            kd.insert(p)

        # Test RTree
        rt = RTree2D(4)
        for p in points:
            rt.insert(p)

        # Test RStarTree
        rst = RStarTree2D(4)
        for p in points:
            rst.insert(p)

        # All should find the same nearest neighbor
        query = Point2D(11.0, 21.0, None)
        qt_result = qt.knn_search(query, 1)[0]
        kd_result = kd.knn_search(query, 1)[0]
        rt_result = rt.knn_search(query, 1)[0]
        rst_result = rst.knn_search(query, 1)[0]

        # All should return the same point (p1)
        assert qt_result.data == "p1"
        assert kd_result.data == "p1"
        assert rt_result.data == "p1"
        assert rst_result.data == "p1"
