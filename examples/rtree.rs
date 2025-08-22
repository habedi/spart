use spart::geometry::{DistanceMetric, EuclideanDistance, Point2D, Point3D};
use spart::r_tree::RTree;

// Define a custom distance metric (Manhattan distance)
struct ManhattanDistance;

impl<T> DistanceMetric<Point2D<T>> for ManhattanDistance {
    fn distance_sq(p1: &Point2D<T>, p2: &Point2D<T>) -> f64 {
        ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()).powi(2)
    }
}

impl<T> DistanceMetric<Point3D<T>> for ManhattanDistance {
    fn distance_sq(p1: &Point3D<T>, p2: &Point3D<T>) -> f64 {
        ((p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()).powi(2)
    }
}

fn main() {
    // --- 2D RTree Example ---
    println!("--- 2D RTree Example ---");
    let mut tree2d: RTree<Point2D<u32>> = RTree::new(4);

    // Insert some points
    tree2d.insert(Point2D::new(10.0, 20.0, Some(1)));
    tree2d.insert(Point2D::new(80.0, 30.0, Some(2)));
    tree2d.insert(Point2D::new(45.0, 70.0, Some(3)));

    // Query the tree for the 2 nearest neighbors to a point
    let query_point_2d = Point2D::new(12.0, 22.0, None);
    let results_2d_euclidean = tree2d.knn_search::<EuclideanDistance>(&query_point_2d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Euclidean): {:?}",
        query_point_2d, results_2d_euclidean
    );

    // Query the tree for the 2 nearest neighbors to a point using Manhattan distance
    let results_2d_manhattan = tree2d.knn_search::<ManhattanDistance>(&query_point_2d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Manhattan): {:?}",
        query_point_2d, results_2d_manhattan
    );

    // --- 3D RTree Example ---
    println!("\n--- 3D RTree Example ---");
    let mut tree3d: RTree<Point3D<u32>> = RTree::new(4);

    // Insert some points
    tree3d.insert(Point3D::new(10.0, 20.0, 30.0, Some(1)));
    tree3d.insert(Point3D::new(80.0, 30.0, 40.0, Some(2)));
    tree3d.insert(Point3D::new(45.0, 70.0, 50.0, Some(3)));

    // Query the tree for the 2 nearest neighbors to a point
    let query_point_3d = Point3D::new(12.0, 22.0, 32.0, None);
    let results_3d_euclidean = tree3d.knn_search::<EuclideanDistance>(&query_point_3d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Euclidean): {:?}",
        query_point_3d, results_3d_euclidean
    );

    // Query the tree for the 2 nearest neighbors to a point using Manhattan distance
    let results_3d_manhattan = tree3d.knn_search::<ManhattanDistance>(&query_point_3d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Manhattan): {:?}",
        query_point_3d, results_3d_manhattan
    );
}
