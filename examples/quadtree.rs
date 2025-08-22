use spart::geometry::{DistanceMetric, EuclideanDistance, Point2D, Rectangle};
use spart::quadtree::Quadtree;

// Define a custom distance metric (Manhattan distance)
struct ManhattanDistance;

impl<T> DistanceMetric<Point2D<T>> for ManhattanDistance {
    fn distance_sq(p1: &Point2D<T>, p2: &Point2D<T>) -> f64 {
        ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()).powi(2)
    }
}

fn main() {
    // Create a new quadtree with a bounding box that spans from (0, 0) to (100, 100)
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut quadtree = Quadtree::<u32>::new(&boundary, 4);

    // Insert some points into the quadtree
    quadtree.insert(Point2D::new(10.0, 20.0, Some(1)));
    quadtree.insert(Point2D::new(80.0, 30.0, Some(2)));
    quadtree.insert(Point2D::new(45.0, 70.0, Some(3)));

    // Query the quadtree for the 2 nearest neighbors to a point
    let query_point = Point2D::new(12.0, 22.0, None);
    let results_euclidean = quadtree.knn_search::<EuclideanDistance>(&query_point, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Euclidean): {:?}",
        query_point, results_euclidean
    );

    // Query the quadtree for the 2 nearest neighbors to a point using Manhattan distance
    let results_manhattan = quadtree.knn_search::<ManhattanDistance>(&query_point, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Manhattan): {:?}",
        query_point, results_manhattan
    );
}
