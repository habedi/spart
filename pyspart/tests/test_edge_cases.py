"""
Test edge cases and error handling for PySpart
"""
import pytest

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_quadtree_zero_capacity_error(self):
        """Test that zero capacity raises an error"""
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        with pytest.raises(ValueError):
            Quadtree(boundary, 0)

    def test_octree_zero_capacity_error(self):
        """Test that zero capacity raises an error"""
        boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
        with pytest.raises(ValueError):
            Octree(boundary, 0)

    def test_rtree_zero_capacity_error(self):
        """Test that zero capacity raises an error"""
        with pytest.raises(ValueError):
            RTree2D(0)
        with pytest.raises(ValueError):
            RTree3D(0)

    def test_rstar_tree_zero_capacity_error(self):
        """Test that zero capacity raises an error"""
        with pytest.raises(ValueError):
            RStarTree2D(0)
        with pytest.raises(ValueError):
            RStarTree3D(0)

    def test_knn_search_k_zero(self):
        """Test kNN search with k=0 returns empty list"""
        kd = KdTree2D()
        kd.insert(Point2D(1.0, 2.0, "data"))
        results = kd.knn_search(Point2D(0.0, 0.0, None), 0)
        assert len(results) == 0

    def test_knn_search_empty_tree(self):
        """Test kNN search on empty tree returns empty list"""
        kd = KdTree2D()
        results = kd.knn_search(Point2D(0.0, 0.0, None), 5)
        assert len(results) == 0

    def test_range_search_zero_radius(self):
        """Test range search with radius=0 finds exact matches"""
        kd = KdTree2D()
        exact_point = Point2D(5.0, 5.0, "exact")
        nearby_point = Point2D(5.1, 5.0, "nearby")

        kd.insert(exact_point)
        kd.insert(nearby_point)

        results = kd.range_search(Point2D(5.0, 5.0, None), 0.0)
        assert len(results) == 1
        assert results[0].data == "exact"

    def test_range_search_empty_tree(self):
        """Test range search on empty tree returns empty list"""
        kd = KdTree2D()
        results = kd.range_search(Point2D(0.0, 0.0, None), 10.0)
        assert len(results) == 0

    def test_delete_nonexistent_point(self):
        """Test deleting a point that doesn't exist returns False"""
        kd = KdTree2D()
        kd.insert(Point2D(1.0, 1.0, "exists"))

        result = kd.delete(Point2D(99.0, 99.0, "nonexistent"))
        assert result == False

    def test_delete_from_empty_tree(self):
        """Test deleting from empty tree returns False"""
        kd = KdTree2D()
        result = kd.delete(Point2D(1.0, 1.0, "data"))
        assert result == False

    def test_quadtree_out_of_bounds_insert(self):
        """Test inserting point outside quadtree boundary"""
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        qt = Quadtree(boundary, 4)

        # Point outside boundary should not be inserted
        result = qt.insert(Point2D(150.0, 150.0, "outside"))
        assert result == False

        # Verify it wasn't inserted
        results = qt.knn_search(Point2D(150.0, 150.0, None), 1)
        assert len(results) == 0

    def test_octree_out_of_bounds_insert(self):
        """Test inserting point outside octree boundary"""
        boundary = {"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}
        ot = Octree(boundary, 4)

        # Point outside boundary should not be inserted
        result = ot.insert(Point3D(150.0, 150.0, 150.0, "outside"))
        assert result == False

    def test_bulk_insert_empty_list(self):
        """Test bulk insert with empty list doesn't crash"""
        kd = KdTree2D()
        kd.insert_bulk([])

        # Tree should still be empty
        results = kd.knn_search(Point2D(0.0, 0.0, None), 1)
        assert len(results) == 0

    def test_duplicate_points(self):
        """Test handling of duplicate points"""
        kd = KdTree2D()
        p1 = Point2D(5.0, 5.0, "first")
        p2 = Point2D(5.0, 5.0, "second")
        p3 = Point2D(5.0, 5.0, "third")

        kd.insert(p1)
        kd.insert(p2)
        kd.insert(p3)

        # All duplicates should be stored
        results = kd.knn_search(Point2D(5.0, 5.0, None), 3)
        assert len(results) == 3

    def test_point_on_boundary(self):
        """Test points exactly on quadtree boundary"""
        boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
        qt = Quadtree(boundary, 4)

        # Points on boundary should be included
        corner_points = [
            Point2D(0.0, 0.0, "corner1"),
            Point2D(100.0, 100.0, "corner2"),
            Point2D(0.0, 100.0, "corner3"),
            Point2D(100.0, 0.0, "corner4"),
        ]

        for p in corner_points:
            result = qt.insert(p)
            assert result == True

    def test_large_k_value(self):
        """Test kNN search with k larger than number of points"""
        kd = KdTree2D()
        kd.insert(Point2D(1.0, 1.0, "p1"))
        kd.insert(Point2D(2.0, 2.0, "p2"))

        # Request more neighbors than available
        results = kd.knn_search(Point2D(0.0, 0.0, None), 100)
        assert len(results) == 2  # Should return all available points

    def test_negative_coordinates(self):
        """Test handling of negative coordinates"""
        kd = KdTree2D()
        kd.insert(Point2D(-10.0, -20.0, "negative"))
        kd.insert(Point2D(10.0, 20.0, "positive"))

        results = kd.knn_search(Point2D(-10.0, -20.0, None), 1)
        assert len(results) == 1
        assert results[0].data == "negative"

    def test_very_large_coordinates(self):
        """Test handling of very large coordinate values"""
        kd = KdTree2D()
        kd.insert(Point2D(1e10, 1e10, "huge"))
        kd.insert(Point2D(1e10 + 1, 1e10 + 1, "huge2"))

        results = kd.knn_search(Point2D(1e10, 1e10, None), 1)
        assert len(results) == 1
        assert results[0].data == "huge"

    def test_mixed_data_types(self):
        """Test that points can store different types of data"""
        kd = KdTree2D()
        kd.insert(Point2D(1.0, 1.0, "string"))
        kd.insert(Point2D(2.0, 2.0, 123))
        kd.insert(Point2D(3.0, 3.0, {"key": "value"}))
        kd.insert(Point2D(4.0, 4.0, [1, 2, 3]))
        kd.insert(Point2D(5.0, 5.0, None))

        results = kd.knn_search(Point2D(3.0, 3.0, None), 1)
        assert results[0].data == {"key": "value"}


class TestDataIntegrity:
    """Test data integrity across operations"""

    def test_data_preserved_after_operations(self):
        """Test that point data is preserved through various operations"""
        data = {"id": 123, "name": "test_point", "metadata": {"type": "sensor"}}
        point = Point2D(10.0, 20.0, data)

        kd = KdTree2D()
        kd.insert(point)

        # Retrieve and verify data
        results = kd.knn_search(Point2D(10.0, 20.0, None), 1)
        assert results[0].data == data

    def test_coordinates_preserved(self):
        """Test that coordinates are exactly preserved"""
        x, y = 12.3456789, 98.7654321
        point = Point2D(x, y, "data")

        kd = KdTree2D()
        kd.insert(point)

        results = kd.knn_search(Point2D(x, y, None), 1)
        assert results[0].x == x
        assert results[0].y == y

    def test_knn_returns_correct_distances(self):
        """Test that kNN search returns points in correct distance order"""
        kd = KdTree2D()
        kd.insert(Point2D(0.0, 0.0, "origin"))
        kd.insert(Point2D(1.0, 0.0, "close"))
        kd.insert(Point2D(10.0, 0.0, "far"))

        target = Point2D(0.0, 0.0, None)
        results = kd.knn_search(target, 3)

        # Calculate distances
        distances = []
        for p in results:
            dist = ((p.x - target.x)**2 + (p.y - target.y)**2)**0.5
            distances.append(dist)

        # Verify sorted by distance
        assert distances == sorted(distances)

    def test_range_search_radius_boundary(self):
        """Test that range search correctly handles points on radius boundary"""
        kd = KdTree2D()
        kd.insert(Point2D(0.0, 0.0, "center"))
        kd.insert(Point2D(10.0, 0.0, "on_boundary"))  # Exactly 10 units away
        kd.insert(Point2D(10.1, 0.0, "outside"))      # Just outside

        results = kd.range_search(Point2D(0.0, 0.0, None), 10.0)

        # Should include center and boundary point
        data_list = [p.data for p in results]
        assert "center" in data_list
        assert "on_boundary" in data_list
        assert "outside" not in data_list

