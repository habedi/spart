"""
Test advanced use cases and real-world scenarios for pyspart
"""
import pytest
import math

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


class TestRealWorldScenarios:
    """Test realistic use cases"""

    def test_geospatial_nearest_neighbor(self):
        """Test finding nearest city to a location"""
        # Simulate cities with coordinates
        cities = [
            Point2D(40.7128, -74.0060, {"name": "New York", "population": 8336817}),
            Point2D(34.0522, -118.2437, {"name": "Los Angeles", "population": 3979576}),
            Point2D(41.8781, -87.6298, {"name": "Chicago", "population": 2693976}),
            Point2D(29.7604, -95.3698, {"name": "Houston", "population": 2320268}),
            Point2D(33.4484, -112.0740, {"name": "Phoenix", "population": 1680992}),
        ]

        kd = KdTree2D()
        kd.insert_bulk(cities)

        # Find nearest city to a point
        location = Point2D(40.0, -75.0, None)  # Near New York
        nearest = kd.knn_search(location, 1)[0]

        assert nearest.data["name"] == "New York"

    def test_range_query_for_area_search(self):
        """Test finding all points within a geographic area"""
        # Simulate sensor locations
        sensors = [
            Point2D(0.0, 0.0, {"id": 1, "type": "temperature"}),
            Point2D(5.0, 5.0, {"id": 2, "type": "humidity"}),
            Point2D(10.0, 10.0, {"id": 3, "type": "pressure"}),
            Point2D(15.0, 15.0, {"id": 4, "type": "temperature"}),
            Point2D(50.0, 50.0, {"id": 5, "type": "humidity"}),
        ]

        qt = Quadtree({"x": -100.0, "y": -100.0, "width": 200.0, "height": 200.0}, 4)
        for sensor in sensors:
            qt.insert(sensor)

        # Find sensors within 8 units of origin
        center = Point2D(0.0, 0.0, None)
        nearby_sensors = qt.range_search(center, 8.0)

        # Should find sensors 1 and 2
        ids = [s.data["id"] for s in nearby_sensors]
        assert 1 in ids
        assert 2 in ids
        assert 5 not in ids

    def test_clustering_points(self):
        """Test finding clusters of points"""
        # Create three clusters of points
        cluster1 = [Point2D(10.0 + i*0.5, 10.0 + i*0.5, f"c1_{i}") for i in range(10)]
        cluster2 = [Point2D(50.0 + i*0.5, 50.0 + i*0.5, f"c2_{i}") for i in range(10)]
        cluster3 = [Point2D(90.0 + i*0.5, 90.0 + i*0.5, f"c3_{i}") for i in range(10)]

        all_points = cluster1 + cluster2 + cluster3

        kd = KdTree2D()
        kd.insert_bulk(all_points)

        # Find points in cluster 1
        cluster1_center = Point2D(12.5, 12.5, None)
        cluster1_found = kd.range_search(cluster1_center, 5.0)

        # Should find most of cluster 1 points
        c1_data = [p.data for p in cluster1_found if p.data.startswith("c1_")]
        assert len(c1_data) >= 8  # Most of cluster 1

    def test_3d_spatial_indexing(self):
        """Test 3D spatial indexing for voxel data"""
        # Simulate 3D voxel grid
        voxels = []
        for x in range(10):
            for y in range(10):
                for z in range(10):
                    voxels.append(Point3D(
                        float(x), float(y), float(z),
                        {"voxel_id": f"{x}_{y}_{z}", "density": x + y + z}
                    ))

        octree = Octree({"x": 0.0, "y": 0.0, "z": 0.0, "width": 10.0, "height": 10.0, "depth": 10.0}, 8)
        octree.insert_bulk(voxels)

        # Find voxels near a point
        target = Point3D(5.0, 5.0, 5.0, None)
        nearby_voxels = octree.knn_search(target, 5)

        assert len(nearby_voxels) == 5

        # Verify they are actually close
        for voxel in nearby_voxels:
            dist = math.sqrt((voxel.x - 5.0)**2 + (voxel.y - 5.0)**2 + (voxel.z - 5.0)**2)
            assert dist < 3.0  # Should be relatively close

    def test_dynamic_updates(self):
        """Test handling dynamic point updates (insert and delete)"""
        kd = KdTree2D()

        # Initial points
        initial_points = [Point2D(float(i*10), float(i*10), f"init_{i}") for i in range(10)]
        kd.insert_bulk(initial_points)

        # Remove some points (simulate objects moving out of area)
        for point in initial_points[:3]:
            kd.delete(point)

        # Add new points (simulate new objects entering area)
        new_points = [Point2D(float(i*10 + 5), float(i*10 + 5), f"new_{i}") for i in range(5)]
        for point in new_points:
            kd.insert(point)

        # Verify current state
        all_points = kd.knn_search(Point2D(50.0, 50.0, None), 100)
        assert len(all_points) == 12  # 7 initial + 5 new

    def test_path_planning_waypoints(self):
        """Test finding waypoints along a path"""
        # Simulate waypoints along a route
        waypoints = [Point2D(float(i*10), float(i*5), f"waypoint_{i}") for i in range(20)]

        kd = KdTree2D()
        kd.insert_bulk(waypoints)

        # Find next 3 waypoints from current position
        current_pos = Point2D(55.0, 27.0, None)
        next_waypoints = kd.knn_search(current_pos, 3)

        assert len(next_waypoints) == 3

        # Verify they are sequential waypoints
        indices = [int(wp.data.split("_")[1]) for wp in next_waypoints]
        assert max(indices) - min(indices) <= 2  # Should be consecutive or near

    def test_collision_detection(self):
        """Test finding potential collisions in a region"""
        # Simulate objects with positions
        objects = [
            Point2D(10.0, 10.0, {"id": "obj1", "radius": 2.0}),
            Point2D(10.5, 10.5, {"id": "obj2", "radius": 1.5}),  # Overlapping with obj1
            Point2D(50.0, 50.0, {"id": "obj3", "radius": 3.0}),
            Point2D(51.0, 51.0, {"id": "obj4", "radius": 2.0}),  # Overlapping with obj3
        ]

        rt = RTree2D(4)
        for obj in objects:
            rt.insert(obj)

        # Check for potential collisions around obj1
        obj1_pos = Point2D(10.0, 10.0, None)
        obj1_radius = 2.0

        nearby = rt.range_search(obj1_pos, obj1_radius * 2)  # Check double the radius

        # Should find obj1 and obj2
        ids = [o.data["id"] for o in nearby]
        assert "obj1" in ids
        assert "obj2" in ids
        assert "obj3" not in ids


class TestAdvancedQueries:
    """Test advanced query patterns"""

    def test_incremental_knn(self):
        """Test getting nearest neighbors incrementally"""
        kd = KdTree2D()
        points = [Point2D(float(i), float(i), f"p{i}") for i in range(20)]
        kd.insert_bulk(points)

        target = Point2D(10.0, 10.0, None)

        # Get nearest 5, then 10, then 15
        nearest_5 = kd.knn_search(target, 5)
        nearest_10 = kd.knn_search(target, 10)
        nearest_15 = kd.knn_search(target, 15)

        # Verify the counts are correct
        assert len(nearest_5) == 5
        assert len(nearest_10) == 10
        assert len(nearest_15) == 15

        # First 5 of nearest_10 should be among the nearest_5 (order may vary with equal distances)
        import math
        nearest_5_dists = sorted([math.sqrt((p.x - target.x)**2 + (p.y - target.y)**2) for p in nearest_5])
        nearest_10_first_5_dists = sorted([math.sqrt((p.x - target.x)**2 + (p.y - target.y)**2) for p in nearest_10[:5]])

        # The 5th farthest distance in nearest_5 should match the 5th in nearest_10
        assert abs(nearest_5_dists[-1] - nearest_10_first_5_dists[-1]) < 0.001

    def test_multi_range_query(self):
        """Test multiple overlapping range queries"""
        qt = Quadtree({"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}, 4)
        points = [Point2D(float(i), float(i), f"p{i}") for i in range(50)]
        qt.insert_bulk(points)

        # Multiple range queries at different centers
        centers = [
            Point2D(10.0, 10.0, None),
            Point2D(30.0, 30.0, None),
            Point2D(50.0, 50.0, None),
        ]

        all_results = set()
        for center in centers:
            results = qt.range_search(center, 5.0)
            for r in results:
                all_results.add(r.data)

        # Should have found points from all regions
        assert len(all_results) > 0

    def test_filtered_search(self):
        """Test searching with data-based filtering"""
        kd = KdTree2D()
        points = [
            Point2D(10.0, 10.0, {"type": "A", "value": 100}),
            Point2D(11.0, 11.0, {"type": "B", "value": 200}),
            Point2D(12.0, 12.0, {"type": "A", "value": 150}),
            Point2D(13.0, 13.0, {"type": "B", "value": 250}),
            Point2D(14.0, 14.0, {"type": "A", "value": 300}),
        ]
        kd.insert_bulk(points)

        # Find all nearest neighbors and filter by type
        target = Point2D(12.0, 12.0, None)
        all_neighbors = kd.knn_search(target, 5)

        type_a_neighbors = [p for p in all_neighbors if p.data["type"] == "A"]

        assert len(type_a_neighbors) == 3
        assert all(p.data["type"] == "A" for p in type_a_neighbors)

    def test_weighted_search(self):
        """Test finding points with weighted criteria"""
        kd = KdTree2D()
        points = [
            Point2D(10.0, 10.0, {"priority": 1}),
            Point2D(20.0, 20.0, {"priority": 5}),
            Point2D(30.0, 30.0, {"priority": 3}),
            Point2D(40.0, 40.0, {"priority": 2}),
        ]
        kd.insert_bulk(points)

        target = Point2D(25.0, 25.0, None)
        neighbors = kd.knn_search(target, 4)

        # Could rank by combination of distance and priority
        # Here we just verify all neighbors are retrieved
        assert len(neighbors) == 4
        priorities = [p.data["priority"] for p in neighbors]
        assert max(priorities) == 5
        assert min(priorities) == 1
"""
Test performance and scalability of pyspart data structures
"""
import pytest
import time

from pyspart import (
    Point2D, Point3D,
    Quadtree, Octree,
    KdTree2D, KdTree3D,
    RTree2D, RTree3D,
    RStarTree2D, RStarTree3D
)


@pytest.fixture
def small_2d_dataset():
    """Generate a small 2D dataset"""
    return [Point2D(float(i), float(i * 2), f"point_{i}") for i in range(100)]


@pytest.fixture
def medium_2d_dataset():
    """Generate a medium 2D dataset"""
    return [Point2D(float(i % 100), float(i // 100), f"point_{i}") for i in range(1000)]


@pytest.fixture
def small_3d_dataset():
    """Generate a small 3D dataset"""
    return [Point3D(float(i), float(i * 2), float(i * 3), f"point_{i}") for i in range(100)]


class TestPerformance:
    """Test performance characteristics"""

    def test_kdtree2d_bulk_vs_individual_insert(self, small_2d_dataset):
        """Compare bulk insert vs individual inserts"""
        # Bulk insert
        tree1 = KdTree2D()
        start = time.time()
        tree1.insert_bulk(small_2d_dataset)
        bulk_time = time.time() - start

        # Individual inserts
        tree2 = KdTree2D()
        start = time.time()
        for point in small_2d_dataset:
            tree2.insert(point)
        individual_time = time.time() - start

        # Both should have same number of points
        results1 = tree1.knn_search(Point2D(0.0, 0.0, None), len(small_2d_dataset))
        results2 = tree2.knn_search(Point2D(0.0, 0.0, None), len(small_2d_dataset))
        assert len(results1) == len(results2)

        # Note: Bulk insert may or may not be faster depending on implementation
        print(f"Bulk insert: {bulk_time:.4f}s, Individual: {individual_time:.4f}s")

    def test_knn_search_performance(self, medium_2d_dataset):
        """Test kNN search performance with different k values"""
        kd = KdTree2D()
        kd.insert_bulk(medium_2d_dataset)

        target = Point2D(50.0, 50.0, None)

        # Test different k values
        for k in [1, 10, 50, 100]:
            start = time.time()
            results = kd.knn_search(target, k)
            elapsed = time.time() - start

            assert len(results) == k
            print(f"kNN search with k={k}: {elapsed:.4f}s")

    def test_range_search_performance(self, medium_2d_dataset):
        """Test range search performance with different radii"""
        kd = KdTree2D()
        kd.insert_bulk(medium_2d_dataset)

        center = Point2D(50.0, 50.0, None)

        # Test different radii
        for radius in [5.0, 10.0, 20.0, 50.0]:
            start = time.time()
            results = kd.range_search(center, radius)
            elapsed = time.time() - start

            print(f"Range search with radius={radius}: {elapsed:.4f}s, found {len(results)} points")

    def test_multiple_operations_sequence(self, small_2d_dataset):
        """Test performance of mixed operations"""
        kd = KdTree2D()

        # Insert
        for point in small_2d_dataset[:50]:
            kd.insert(point)

        # Search
        for i in range(10):
            kd.knn_search(Point2D(float(i), float(i), None), 5)

        # More inserts
        for point in small_2d_dataset[50:]:
            kd.insert(point)

        # More searches
        for i in range(10):
            kd.range_search(Point2D(float(i * 5), float(i * 5), None), 10.0)

        # Verify final state
        results = kd.knn_search(Point2D(0.0, 0.0, None), 100)
        assert len(results) == 100


class TestScalability:
    """Test scalability with larger datasets"""

    def test_large_dataset_insert(self):
        """Test inserting a large number of points"""
        kd = KdTree2D()
        num_points = 5000

        points = [Point2D(float(i % 100), float(i // 100), i) for i in range(num_points)]

        start = time.time()
        kd.insert_bulk(points)
        elapsed = time.time() - start

        print(f"Inserted {num_points} points in {elapsed:.4f}s")

        # Verify some points are findable
        results = kd.knn_search(Point2D(50.0, 50.0, None), 10)
        assert len(results) == 10

    def test_quadtree_subdivision_performance(self):
        """Test quadtree performance with many subdivisions"""
        boundary = {"x": 0.0, "y": 0.0, "width": 1000.0, "height": 1000.0}
        qt = Quadtree(boundary, 4)  # Small capacity to force subdivisions

        # Insert points in a pattern that will cause many subdivisions
        num_points = 1000
        points = [Point2D(float(i % 100) * 10, float(i // 100) * 10, i) for i in range(num_points)]

        start = time.time()
        for point in points:
            qt.insert(point)
        elapsed = time.time() - start

        print(f"Quadtree with subdivisions: {elapsed:.4f}s for {num_points} points")

        # Verify points are retrievable
        results = qt.knn_search(Point2D(500.0, 500.0, None), 10)
        assert len(results) == 10

    def test_delete_performance(self, small_2d_dataset):
        """Test deletion performance"""
        kd = KdTree2D()
        kd.insert_bulk(small_2d_dataset)

        # Delete half the points
        start = time.time()
        deleted_count = 0
        for point in small_2d_dataset[:50]:
            if kd.delete(point):
                deleted_count += 1
        elapsed = time.time() - start

        print(f"Deleted {deleted_count} points in {elapsed:.4f}s")

        # Verify deletions worked
        results = kd.knn_search(Point2D(0.0, 0.0, None), 100)
        assert len(results) == 50  # Half remaining


class TestTreeComparison:
    """Compare different tree implementations"""

    def test_kdtree_vs_quadtree_2d(self, small_2d_dataset):
        """Compare KdTree and Quadtree for 2D data"""
        # Setup KdTree
        kd = KdTree2D()
        kd.insert_bulk(small_2d_dataset)

        # Setup Quadtree
        boundary = {"x": 0.0, "y": 0.0, "width": 200.0, "height": 200.0}
        qt = Quadtree(boundary, 4)
        qt.insert_bulk(small_2d_dataset)

        # Compare kNN results
        target = Point2D(50.0, 50.0, None)
        kd_results = kd.knn_search(target, 10)
        qt_results = qt.knn_search(target, 10)

        assert len(kd_results) == len(qt_results)

        # Calculate distances for both
        kd_distances = [((p.x - target.x)**2 + (p.y - target.y)**2)**0.5 for p in kd_results]
        qt_distances = [((p.x - target.x)**2 + (p.y - target.y)**2)**0.5 for p in qt_results]

        # Both should return similar nearest neighbors
        assert abs(sum(kd_distances) - sum(qt_distances)) < 1.0

    def test_rtree_vs_rstar_tree_2d(self, small_2d_dataset):
        """Compare RTree and RStarTree for 2D data"""
        # Setup RTree
        rt = RTree2D(4)
        rt.insert_bulk(small_2d_dataset)

        # Setup RStarTree
        rst = RStarTree2D(4)
        rst.insert_bulk(small_2d_dataset)

        # Compare range search results
        center = Point2D(50.0, 100.0, None)
        radius = 20.0

        rt_results = rt.range_search(center, radius)
        rst_results = rst.range_search(center, radius)

        # Both should find the same points (or very similar)
        assert abs(len(rt_results) - len(rst_results)) <= 1

    def test_all_trees_consistency_2d(self):
        """Verify all 2D tree types return consistent results"""
        points = [
            Point2D(10.0, 10.0, "p1"),
            Point2D(20.0, 20.0, "p2"),
            Point2D(30.0, 30.0, "p3"),
            Point2D(40.0, 40.0, "p4"),
        ]

        # Create all tree types
        kd = KdTree2D()
        qt = Quadtree({"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}, 4)
        rt = RTree2D(4)
        rst = RStarTree2D(4)

        # Insert same data
        for point in points:
            kd.insert(point)
            qt.insert(point)
            rt.insert(point)
            rst.insert(point)

        # Query all trees
        target = Point2D(25.0, 25.0, None)
        kd_result = kd.knn_search(target, 1)[0]
        qt_result = qt.knn_search(target, 1)[0]
        rt_result = rt.knn_search(target, 1)[0]
        rst_result = rst.knn_search(target, 1)[0]

        # Calculate distances to verify all found the nearest neighbor (may be different due to ties)
        import math
        kd_dist = math.sqrt((kd_result.x - target.x)**2 + (kd_result.y - target.y)**2)
        qt_dist = math.sqrt((qt_result.x - target.x)**2 + (qt_result.y - target.y)**2)
        rt_dist = math.sqrt((rt_result.x - target.x)**2 + (rt_result.y - target.y)**2)
        rst_dist = math.sqrt((rst_result.x - target.x)**2 + (rst_result.y - target.y)**2)

        # All should return points at the same distance (p2 and p3 are equidistant from target)
        assert abs(kd_dist - qt_dist) < 0.001
        assert abs(kd_dist - rt_dist) < 0.001


    def test_all_trees_consistency_3d(self):
        """Verify all 3D tree types return consistent results"""
        points = [
            Point3D(10.0, 10.0, 10.0, "p1"),
            Point3D(20.0, 20.0, 20.0, "p2"),
            Point3D(30.0, 30.0, 30.0, "p3"),
        ]

        # Create all 3D tree types
        kd = KdTree3D()
        ot = Octree({"x": 0.0, "y": 0.0, "z": 0.0, "width": 100.0, "height": 100.0, "depth": 100.0}, 4)
        rt = RTree3D(4)
        rst = RStarTree3D(4)

        # Insert same data
        for point in points:
            kd.insert(point)
            ot.insert(point)
            rt.insert(point)
            rst.insert(point)

        # Query all trees
        target = Point3D(15.0, 15.0, 15.0, None)
        kd_result = kd.knn_search(target, 1)[0]
        ot_result = ot.knn_search(target, 1)[0]
        rt_result = rt.knn_search(target, 1)[0]
        rst_result = rst.knn_search(target, 1)[0]

        # Calculate distances to verify all found the nearest neighbor
        import math
        kd_dist = math.sqrt((kd_result.x - target.x)**2 + (kd_result.y - target.y)**2 + (kd_result.z - target.z)**2)
        ot_dist = math.sqrt((ot_result.x - target.x)**2 + (ot_result.y - target.y)**2 + (ot_result.z - target.z)**2)
        rt_dist = math.sqrt((rt_result.x - target.x)**2 + (rt_result.y - target.y)**2 + (rt_result.z - target.z)**2)
        rst_dist = math.sqrt((rst_result.x - target.x)**2 + (rst_result.y - target.y)**2 + (rst_result.z - target.z)**2)

        # All should return points at the same distance
        assert abs(kd_dist - ot_dist) < 0.001
        assert abs(kd_dist - rt_dist) < 0.001
        assert abs(kd_dist - rst_dist) < 0.001

