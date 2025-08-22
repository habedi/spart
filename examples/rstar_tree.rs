use spart::geometry::{Point2D, Point3D};
use spart::r_star_tree::RStarTree;

fn main() {
    // Create a new R*-tree for 2D points with a maximum capacity of 4 points per node.
    let mut tree_2d = RStarTree::new(4);
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
    let neighbors_2d = tree_2d.knn_search(&point1_2d, 2);
    println!("kNN search results for {:?}: {:?}", point1_2d, neighbors_2d);

    // Perform a range search with a radius of 5.0.
    let range_points_2d = tree_2d.range_search(&point1_2d, 5.0);
    println!(
        "Range search results for {:?}: {:?}",
        point1_2d, range_points_2d
    );

    // Remove a point from the tree.
    tree_2d.delete(&point1_2d);
    println!("Deleted point: {:?}", point1_2d);

    // Create a new R*-tree for 3D points with a maximum capacity of 4 points per node.
    let mut tree_3d = RStarTree::new(4);
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
    let neighbors_3d = tree_3d.knn_search(&point1_3d, 2);
    println!("kNN search results for {:?}: {:?}", point1_3d, neighbors_3d);

    // Perform a range search with a radius of 5.0.
    let range_points_3d = tree_3d.range_search(&point1_3d, 5.0);
    println!(
        "Range search results for {:?}: {:?}",
        point1_3d, range_points_3d
    );

    // Remove a point from the tree.
    tree_3d.delete(&point1_3d);
    println!("Deleted point: {:?}", point1_3d);
}
