use spart::geometry::{Cube, Point3D};
use spart::octree::Octree;

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
    let results = octree.knn_search(&query_point, 2);

    // Print the results
    println!("2 nearest neighbors to {:?}: {:?}", query_point, results);
}
