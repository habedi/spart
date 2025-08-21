use spart::geometry::{Point2D, Point3D};
use spart::kd_tree::KdTree;

fn main() {
    // --- 2D KdTree Example ---
    println!("--- 2D KdTree Example ---");
    let mut tree2d: KdTree<Point2D<u32>> = KdTree::new(2);

    // Insert some points
    tree2d.insert(Point2D::new(10.0, 20.0, Some(1)));
    tree2d.insert(Point2D::new(80.0, 30.0, Some(2)));
    tree2d.insert(Point2D::new(45.0, 70.0, Some(3)));

    // Query the tree for the 2 nearest neighbors to a point
    let query_point_2d = Point2D::new(12.0, 22.0, None);
    let results_2d = tree2d.knn_search(&query_point_2d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?}: {:?}",
        query_point_2d, results_2d
    );

    // --- 3D KdTree Example ---
    println!("\n--- 3D KdTree Example ---");
    let mut tree3d: KdTree<Point3D<u32>> = KdTree::new(3);

    // Insert some points
    tree3d.insert(Point3D::new(10.0, 20.0, 30.0, Some(1)));
    tree3d.insert(Point3D::new(80.0, 30.0, 40.0, Some(2)));
    tree3d.insert(Point3D::new(45.0, 70.0, 50.0, Some(3)));

    // Query the tree for the 2 nearest neighbors to a point
    let query_point_3d = Point3D::new(12.0, 22.0, 32.0, None);
    let results_3d = tree3d.knn_search(&query_point_3d, 2);

    // Print the results
    println!(
        "2 nearest neighbors to {:?}: {:?}",
        query_point_3d, results_3d
    );
}
