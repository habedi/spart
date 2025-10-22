//! Regression tests for bug fixes
//!
//! This file contains tests for specific bugs found in the codebase and their fixes.

use spart::geometry::{Cube, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::kdtree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;

/// Bug: Rectangle intersection should handle edge-touching cases correctly
/// The intersects() method uses `<` instead of `<=` which could miss edge-touching rectangles
#[test]
fn test_regression_rectangle_edge_touching() {
    // Two rectangles that touch at an edge
    let r1 = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    let r2 = Rectangle {
        x: 10.0, // Touches r1's right edge
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    // Edge-touching rectangles should be considered as intersecting
    // (common convention in computational geometry)
    assert!(
        r1.intersects(&r2),
        "Edge-touching rectangles should intersect"
    );

    // Test vertical edge touching
    let r3 = Rectangle {
        x: 0.0,
        y: 10.0, // Touches r1's bottom edge
        width: 10.0,
        height: 10.0,
    };
    assert!(
        r1.intersects(&r3),
        "Vertically edge-touching rectangles should intersect"
    );
}

/// Bug: Cube intersection should handle edge-touching cases correctly
#[test]
fn test_regression_cube_edge_touching() {
    let c1 = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };
    let c2 = Cube {
        x: 10.0, // Touches c1's edge
        y: 0.0,
        z: 0.0,
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };

    assert!(c1.intersects(&c2), "Edge-touching cubes should intersect");
}

/// Bug: Rectangle contains should handle boundary points correctly
#[test]
fn test_regression_rectangle_boundary_contains() {
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    // Points on the boundary should be contained
    let p1: Point2D<()> = Point2D::new(0.0, 0.0, None); // Top-left corner
    let p2: Point2D<()> = Point2D::new(10.0, 10.0, None); // Bottom-right corner
    let p3: Point2D<()> = Point2D::new(5.0, 0.0, None); // Top edge
    let p4: Point2D<()> = Point2D::new(10.0, 5.0, None); // Right edge

    assert!(rect.contains(&p1), "Top-left corner should be contained");
    assert!(
        rect.contains(&p2),
        "Bottom-right corner should be contained"
    );
    assert!(rect.contains(&p3), "Top edge point should be contained");
    assert!(rect.contains(&p4), "Right edge point should be contained");
}

/// Bug: Cube contains should handle boundary points correctly
#[test]
fn test_regression_cube_boundary_contains() {
    let cube = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };

    // Points on the boundary should be contained
    let p1: Point3D<()> = Point3D::new(0.0, 0.0, 0.0, None);
    let p2: Point3D<()> = Point3D::new(10.0, 10.0, 10.0, None);
    let p3: Point3D<()> = Point3D::new(5.0, 5.0, 0.0, None);

    assert!(cube.contains(&p1), "Corner should be contained");
    assert!(cube.contains(&p2), "Opposite corner should be contained");
    assert!(cube.contains(&p3), "Face point should be contained");
}

/// Bug: KdTree delete with only left subtree doesn't maintain invariant
/// When deleting a node with only a left subtree, the entire left subtree is promoted,
/// which may violate the KdTree splitting plane invariant
#[test]
fn test_regression_kdtree_delete_left_subtree_only() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::new();

    // Create a specific structure: root with only left children
    let p1 = Point2D::new(50.0, 50.0, Some(1));
    let p2 = Point2D::new(25.0, 25.0, Some(2)); // Goes left
    let p3 = Point2D::new(20.0, 30.0, Some(3)); // Goes left of p2

    tree.insert(p1.clone()).unwrap();
    tree.insert(p2.clone()).unwrap();
    tree.insert(p3.clone()).unwrap();

    // Delete root - this will promote left subtree
    let deleted = tree.delete(&p1);
    assert!(deleted, "Should delete the root");

    // The remaining points should still be findable
    let results = tree.knn_search::<EuclideanDistance>(&p2, 1);
    assert!(!results.is_empty(), "Should find remaining points");

    let results = tree.knn_search::<EuclideanDistance>(&p3, 1);
    assert!(!results.is_empty(), "Should find remaining points");
}

/// Bug: Quadtree delete only removes first occurrence
/// If the same point is inserted multiple times, delete only removes one occurrence
#[test]
fn test_regression_quadtree_delete_duplicates() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = Quadtree::new(&boundary, 10).unwrap();

    let point = Point2D::new(50.0, 50.0, Some(1));

    // Insert the same point multiple times
    tree.insert(point.clone());
    tree.insert(point.clone());
    tree.insert(point.clone());

    // First delete should succeed
    assert!(tree.delete(&point), "First delete should succeed");

    // After one delete, we should still be able to find the point (duplicates exist)
    let results = tree.knn_search::<EuclideanDistance>(&point, 1);
    assert!(!results.is_empty(), "Should still find duplicate points");

    // Second delete should also succeed
    assert!(tree.delete(&point), "Second delete should succeed");
}

/// Bug: Octree delete only removes first occurrence
#[test]
fn test_regression_octree_delete_duplicates() {
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };
    let mut tree = Octree::new(&boundary, 10).unwrap();

    let point = Point3D::new(50.0, 50.0, 50.0, Some(1));

    // Insert the same point multiple times
    tree.insert(point.clone());
    tree.insert(point.clone());
    tree.insert(point.clone());

    // First delete should succeed
    assert!(tree.delete(&point), "First delete should succeed");

    // After one delete, we should still be able to find the point
    let results = tree.knn_search::<EuclideanDistance>(&point, 1);
    assert!(!results.is_empty(), "Should still find duplicate points");
}

/// Bug: Empty KNN search should return empty vector
#[test]
fn test_regression_knn_with_k_zero() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::new();
    tree.insert(Point2D::new(1.0, 1.0, Some(1))).unwrap();

    let target = Point2D::new(0.0, 0.0, Some(0));
    let results = tree.knn_search::<EuclideanDistance>(&target, 0);

    assert_eq!(results.len(), 0, "k=0 should return empty results");
}

/// Bug: KNN search on empty tree should return empty vector
#[test]
fn test_regression_knn_empty_tree() {
    let tree: KdTree<Point2D<i32>> = KdTree::new();
    let target = Point2D::new(0.0, 0.0, Some(0));
    let results = tree.knn_search::<EuclideanDistance>(&target, 5);

    assert_eq!(results.len(), 0, "Empty tree should return empty results");
}

/// Bug: Range search with zero radius should only find exact matches
#[test]
fn test_regression_range_search_zero_radius() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::new();
    let exact_point = Point2D::new(5.0, 5.0, Some(1));
    let nearby_point = Point2D::new(5.1, 5.0, Some(2));

    tree.insert(exact_point.clone()).unwrap();
    tree.insert(nearby_point).unwrap();

    let results = tree.range_search::<EuclideanDistance>(&exact_point, 0.0);

    // Only the exact point should be found with radius 0
    assert_eq!(results.len(), 1, "Zero radius should find only exact match");
    assert_eq!(results[0], exact_point);
}

/// Bug: Quadtree range search should not miss points on the boundary
#[test]
fn test_regression_quadtree_range_boundary() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = Quadtree::new(&boundary, 4).unwrap();

    let center = Point2D::new(50.0, 50.0, Some(1));
    tree.insert(center.clone());

    // Point exactly at radius distance
    let radius = 10.0;
    let boundary_point = Point2D::new(60.0, 50.0, Some(2));
    tree.insert(boundary_point.clone());

    let results = tree.range_search::<EuclideanDistance>(&center, radius);

    // Should find both points (center and boundary point)
    assert!(
        results.len() >= 2,
        "Range search should include points exactly at radius distance"
    );
}

/// Bug: Points outside quadtree boundary should not be inserted
#[test]
fn test_regression_quadtree_out_of_bounds_insert() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = Quadtree::new(&boundary, 4).unwrap();

    // Point outside boundary
    let outside_point = Point2D::new(150.0, 150.0, Some(1));
    let inserted = tree.insert(outside_point.clone());

    assert!(!inserted, "Points outside boundary should not be inserted");

    // Should not be findable
    let results = tree.knn_search::<EuclideanDistance>(&outside_point, 1);
    assert_eq!(results.len(), 0, "Out of bounds point should not be found");
}

/// Bug: Points outside octree boundary should not be inserted
#[test]
fn test_regression_octree_out_of_bounds_insert() {
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };
    let mut tree = Octree::new(&boundary, 4).unwrap();

    // Point outside boundary
    let outside_point = Point3D::new(150.0, 150.0, 150.0, Some(1));
    let inserted = tree.insert(outside_point.clone());

    assert!(!inserted, "Points outside boundary should not be inserted");
}

/// Bug: Deleting non-existent point should return false
#[test]
fn test_regression_delete_nonexistent_point() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::new();
    tree.insert(Point2D::new(1.0, 1.0, Some(1))).unwrap();

    let nonexistent = Point2D::new(99.0, 99.0, Some(99));
    let deleted = tree.delete(&nonexistent);

    assert!(!deleted, "Deleting non-existent point should return false");
}

/// Bug: Bulk insert should handle empty vector
#[test]
fn test_regression_bulk_insert_empty() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::new();
    let empty_vec: Vec<Point2D<i32>> = vec![];

    let result = tree.insert_bulk(empty_vec);
    assert!(
        result.is_ok(),
        "Bulk insert with empty vector should succeed"
    );
}

/// Bug: Quadtree bulk insert should handle empty vector
#[test]
fn test_regression_quadtree_bulk_insert_empty() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = Quadtree::new(&boundary, 4).unwrap();
    let empty_vec: Vec<Point2D<i32>> = vec![];

    tree.insert_bulk(&empty_vec);
    // Should not crash or cause issues
}

/// Bug: Triangle inequality should hold for distances
#[test]
fn test_regression_distance_triangle_inequality() {
    let p1 = Point2D::new(0.0, 0.0, Some(1));
    let p2 = Point2D::new(3.0, 0.0, Some(2));
    let p3 = Point2D::new(3.0, 4.0, Some(3));

    let d12 = p1.distance_sq(&p2).sqrt();
    let d23 = p2.distance_sq(&p3).sqrt();
    let d13 = p1.distance_sq(&p3).sqrt();

    // Triangle inequality: d13 <= d12 + d23
    assert!(d13 <= d12 + d23 + 1e-9, "Triangle inequality should hold");
}

/// Bug: KdTree should handle dimension mismatch gracefully
#[test]
fn test_regression_kdtree_dimension_mismatch() {
    let mut tree: KdTree<Point2D<i32>> = KdTree::with_dimension(2);
    tree.insert(Point2D::new(1.0, 1.0, Some(1))).unwrap();

    // This should work fine - both are 2D points
    let result = tree.insert(Point2D::new(2.0, 2.0, Some(2)));
    assert!(result.is_ok(), "Inserting same dimension should work");
}

/// Bug: Rectangle union with negative coordinates should contain all corners
#[test]
fn test_regression_rectangle_union_negative_coords() {
    let r1 = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 111.08676433386941,
        height: 1.0,
    };
    let r2 = Rectangle {
        x: -191.20362538993982,
        y: 0.0,
        width: 1.0,
        height: 1.0,
    };

    let union = r1.union(&r2);

    // Check all corners of r1
    let r1_min: Point2D<()> = Point2D::new(r1.x, r1.y, None);
    let r1_max: Point2D<()> = Point2D::new(r1.x + r1.width, r1.y + r1.height, None);

    assert!(
        union.contains(&r1_min),
        "Union should contain r1's min corner"
    );
    assert!(
        union.contains(&r1_max),
        "Union should contain r1's max corner at ({}, {}), union is x={}, y={}, w={}, h={}",
        r1_max.x,
        r1_max.y,
        union.x,
        union.y,
        union.width,
        union.height
    );

    // Check all corners of r2
    let r2_min: Point2D<()> = Point2D::new(r2.x, r2.y, None);
    let r2_max: Point2D<()> = Point2D::new(r2.x + r2.width, r2.y + r2.height, None);

    assert!(
        union.contains(&r2_min),
        "Union should contain r2's min corner"
    );
    assert!(
        union.contains(&r2_max),
        "Union should contain r2's max corner"
    );
}

/// Bug: Zero capacity should be rejected
#[test]
fn test_regression_zero_capacity_rejected() {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let result = Quadtree::<i32>::new(&boundary, 0);
    assert!(result.is_err(), "Zero capacity should be rejected");
}

/// Bug: Octree zero capacity should be rejected
#[test]
fn test_regression_octree_zero_capacity_rejected() {
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };

    let result = Octree::<i32>::new(&boundary, 0);
    assert!(result.is_err(), "Zero capacity should be rejected");
}
