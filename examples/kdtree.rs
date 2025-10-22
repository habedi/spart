use spart::geometry::{DistanceMetric, EuclideanDistance, Point2D, Point3D};
use spart::kdtree::KdTree;

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
    // --- 2D KdTree Example ---
    println!("--- 2D KdTree Example ---");
    let mut tree2d: KdTree<Point2D<u32>> = KdTree::new();

    // Insert some points
    tree2d.insert(Point2D::new(10.0, 20.0, Some(1))).unwrap();
    tree2d.insert(Point2D::new(80.0, 30.0, Some(2))).unwrap();
    tree2d.insert(Point2D::new(45.0, 70.0, Some(3))).unwrap();

    // Query the tree for the 2 nearest neighbors to a point using Euclidean distance
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

    // --- 3D KdTree Example ---
    println!("\n--- 3D KdTree Example ---");
    let mut tree3d: KdTree<Point3D<u32>> = KdTree::new();

    // Insert some points
    tree3d
        .insert(Point3D::new(10.0, 20.0, 30.0, Some(1)))
        .unwrap();
    tree3d
        .insert(Point3D::new(80.0, 30.0, 40.0, Some(2)))
        .unwrap();
    tree3d
        .insert(Point3D::new(45.0, 70.0, 50.0, Some(3)))
        .unwrap();

    // Query the tree for the 2 nearest neighbors to a point using Euclidean distance
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

    // Serialize the 2D tree
    let encoded2d: Vec<u8> = bincode::serialize(&tree2d).unwrap();
    let decoded2d: KdTree<Point2D<u32>> = bincode::deserialize(&encoded2d[..]).unwrap();
    let results_2d_decoded = decoded2d.knn_search::<EuclideanDistance>(&query_point_2d, 2);
    println!(
        "2 nearest neighbors to {:?} (Euclidean, decoded): {:?}",
        query_point_2d, results_2d_decoded
    );

    // Serialize the 3D tree
    let encoded3d: Vec<u8> = bincode::serialize(&tree3d).unwrap();
    let decoded3d: KdTree<Point3D<u32>> = bincode::deserialize(&encoded3d[..]).unwrap();
    let results_3d_decoded = decoded3d.knn_search::<EuclideanDistance>(&query_point_3d, 2);
    println!(
        "2 nearest neighbors to {:?} (Euclidean, decoded): {:?}",
        query_point_3d, results_3d_decoded
    );
}
