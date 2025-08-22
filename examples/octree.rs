use spart::geometry::{Cube, DistanceMetric, EuclideanDistance, Point3D};
use spart::octree::Octree;

// Define a custom distance metric (Manhattan distance)
struct ManhattanDistance;

impl<T> DistanceMetric<Point3D<T>> for ManhattanDistance {
    fn distance_sq(p1: &Point3D<T>, p2: &Point3D<T>) -> f64 {
        ((p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()).powi(2)
    }
}

fn main() {
    // Create a new octree with a bounding box that spans from (0, 0, 0) to (100, 100, 100)
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };
    let mut octree = Octree::<u32>::new(&boundary, 4);

    // Insert some points into the octree
    octree.insert(Point3D::new(10.0, 20.0, 30.0, Some(1)));
    octree.insert(Point3D::new(80.0, 30.0, 40.0, Some(2)));
    octree.insert(Point3D::new(45.0, 70.0, 50.0, Some(3)));

    // Query the octree for the 2 nearest neighbors to a point
    let query_point = Point3D::new(12.0, 22.0, 32.0, None);
    let results_euclidean = octree.knn_search::<EuclideanDistance>(&query_point, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Euclidean): {:?}",
        query_point, results_euclidean
    );

    // Query the octree for the 2 nearest neighbors to a point using Manhattan distance
    let results_manhattan = octree.knn_search::<ManhattanDistance>(&query_point, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?} (Manhattan): {:?}",
        query_point, results_manhattan
    );
}
