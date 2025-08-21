use spart::geometry::{Point2D, Rectangle};
use spart::quadtree::Quadtree;

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
    let results = quadtree.knn_search(&query_point, 2);

    // Print the results
    println!("2 nearest neighbors to {:?}: {:?}", query_point, results);
}
