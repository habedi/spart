use spart::geometry::{DistanceMetric, EuclideanDistance, Point2D, Point3D};
use spart::r_star_tree::RStarTree;

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
    // Create a new R*-tree for 2D points with a maximum capacity of 4 points per node.
    let mut tree_2d: RStarTree<Point2D<&str>> = RStarTree::new(4);
    println!("--- 2D R*-Tree Example ---");

    // Define some 2D points.
    let point1_2d = Point2D::new(1.0, 2.0, Some("Point1"));
    let point2_2d = Point2D::new(3.0, 4.0, Some("Point2"));
    let point3_2d = Point2D::new(5.0, 6.0, Some("Point3"));

    // Insert points into the R*-tree.
    tree_2d.insert(point1_2d.clone());
    tree_2d.insert(point2_2d.clone());
    tree_2d.insert(point3_2d.clone());

    // Perform a kNN search.
    let neighbors_2d_euclidean = tree_2d.knn_search::<EuclideanDistance>(&point1_2d, 2);
    println!(
        "kNN search results for {:?} (Euclidean): {:?}",
        point1_2d, neighbors_2d_euclidean
    );

    let neighbors_2d_manhattan = tree_2d.knn_search::<ManhattanDistance>(&point1_2d, 2);
    println!(
        "kNN search results for {:?} (Manhattan): {:?}",
        point1_2d, neighbors_2d_manhattan
    );

    // Perform a range search with a radius of 5.0.
    let range_points_2d = tree_2d.range_search::<EuclideanDistance>(&point1_2d, 5.0);
    println!(
        "Range search results for {:?}: {:?}",
        point1_2d, range_points_2d
    );

    // Remove a point from the tree.
    tree_2d.delete(&point1_2d);
    println!("Deleted point: {:?}", point1_2d);

    // Create a new R*-tree for 3D points with a maximum capacity of 4 points per node.
    let mut tree_3d: RStarTree<Point3D<&str>> = RStarTree::new(4);
    println!("\n--- 3D R*-Tree Example ---");

    // Define some 3D points.
    let point1_3d = Point3D::new(1.0, 2.0, 3.0, Some("Point1"));
    let point2_3d = Point3D::new(4.0, 5.0, 6.0, Some("Point2"));
    let point3_3d = Point3D::new(7.0, 8.0, 9.0, Some("Point3"));

    // Insert points into the R*-tree.
    tree_3d.insert(point1_3d.clone());
    tree_3d.insert(point2_3d.clone());
    tree_3d.insert(point3_3d.clone());

    // Perform a kNN search.
    let neighbors_3d_euclidean = tree_3d.knn_search::<EuclideanDistance>(&point1_3d, 2);
    println!(
        "kNN search results for {:?} (Euclidean): {:?}",
        point1_3d, neighbors_3d_euclidean
    );

    let neighbors_3d_manhattan = tree_3d.knn_search::<ManhattanDistance>(&point1_3d, 2);
    println!(
        "kNN search results for {:?} (Manhattan): {:?}",
        point1_3d, neighbors_3d_manhattan
    );

    // Perform a range search with a radius of 5.0.
    let range_points_3d = tree_3d.range_search::<EuclideanDistance>(&point1_3d, 5.0);
    println!(
        "Range search results for {:?}: {:?}",
        point1_3d, range_points_3d
    );

    // Remove a point from the tree.
    tree_3d.delete(&point1_3d);
    println!("Deleted point: {:?}", point1_3d);
}
