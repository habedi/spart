use spart;
use spart::geometry::{Cube, Point3D};
use spart::geometry::{Point2D, Rectangle};
use spart::kdtree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;
use spart::rtree::RTree;
use tracing::{info, Level};
use tracing_subscriber;

fn main() {
    // If DEBUG_QUADTREE_ZNG is not set or set to false, disable logging. Otherwise, enable logging
    if !std::env::var("DEBUG_QUADTREE_ZNG").is_ok()
        || !std::env::var("DEBUG_QUADTREE_ZNG").is_ok()
            && (std::env::var("DEBUG_QUADTREE_ZNG").unwrap() == "0"
                || std::env::var("DEBUG_QUADTREE_ZNG").unwrap() == "false"
                || std::env::var("DEBUG_QUADTREE_ZNG").unwrap() == "")
    {
        // Disable logging
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }

    // Test Quadtree implementation
    //test_quadtree();

    // Test Octree implementation
    //test_octree();

    // Test KdTree implementation
    //test_kdtree();

    // Test RTree implementation
    test_rtree();
}

fn test_quadtree() {
    println!("{}", "=".repeat(100));
    println!("Quadtree Example");

    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = Quadtree::new(&boundary, 4);

    // Insert some points
    info!("Inserting points into the quadtree");
    tree.insert(Point2D::new(11.0, 11.0, Some("A")));
    tree.insert(Point2D::new(51.0, 51.0, Some("B")));
    tree.insert(Point2D::new(31.0, 41.0, Some("C")));
    tree.insert(Point2D::new(71.0, 81.0, Some("D")));
    tree.insert(Point2D::new(81.0, 91.0, Some("E")));
    tree.insert(Point2D::new(21.0, 21.0, Some("F")));
    tree.insert(Point2D::new(22.0, 22.0, Some("G")));
    tree.insert(Point2D::new(23.0, 23.0, Some("H")));
    tree.insert(Point2D::new(24.0, 24.0, Some("I")));
    tree.insert(Point2D::new(25.0, 25.0, Some("J")));
    tree.insert(Point2D::new(26.0, 26.0, Some("K")));

    // KNN Query
    let target = Point2D::new(35.0, 45.0, None);
    info!("Performing KNN search for target {:?}", target);
    let knn_results = tree.find_closest(&target, 2);
    info!("Nearest Neighbors: {:?}", knn_results);

    // Range Query
    let range_query = Point2D {
        x: 20.0,
        y: 20.0,
        data: None,
    };
    let radius = 30.0;
    info!(
        "Performing range search for point {:?} with radius {}",
        range_query, radius
    );
    let results = tree.find_in_radius(&range_query, radius);
    info!(
        "Points in Range for ({}, {}, r={}):",
        range_query.x, range_query.y, radius
    );
    for point in results {
        info!("({}, {}) -> {:?}", point.x, point.y, point.data);
    }

    // Visualize the tree
    info!("Visualizing the quadtree");
    tree.visualize(0);

    tree.visualize_dot("quadtree.dot");
}

pub fn test_octree() {
    println!("{}", "=".repeat(100));
    println!("Octree Example");

    // Define a cube boundary for the 3D space.
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };

    // Create a new octree with a capacity of 4 points per leaf.
    let mut tree = Octree::new(&boundary, 4);

    // Insert some 3D points.
    info!("Inserting points into the octree");
    tree.insert(Point3D::new(11.0, 11.0, 11.0, Some("A")));
    tree.insert(Point3D::new(51.0, 51.0, 51.0, Some("B")));
    tree.insert(Point3D::new(31.0, 41.0, 21.0, Some("C")));
    tree.insert(Point3D::new(71.0, 81.0, 91.0, Some("D")));
    tree.insert(Point3D::new(81.0, 91.0, 71.0, Some("E")));
    tree.insert(Point3D::new(21.0, 21.0, 21.0, Some("F")));
    tree.insert(Point3D::new(22.0, 22.0, 22.0, Some("G")));
    tree.insert(Point3D::new(23.0, 23.0, 23.0, Some("H")));
    tree.insert(Point3D::new(24.0, 24.0, 24.0, Some("I")));
    tree.insert(Point3D::new(25.0, 25.0, 25.0, Some("J")));
    tree.insert(Point3D::new(26.0, 26.0, 26.0, Some("K")));

    // Perform a K-Nearest Neighbors (KNN) search.
    let target = Point3D::new(35.0, 45.0, 40.0, None);
    info!("Performing KNN search for target {:?}", target);
    let knn_results = tree.find_closest(&target, 2);
    info!("Nearest Neighbors: {:?}", knn_results);

    // Perform a range (radius) query.
    let range_query = Point3D::new(20.0, 20.0, 20.0, None);
    let radius = 30.0;
    info!(
        "Performing range search for point {:?} with radius {}",
        range_query, radius
    );
    let results = tree.find_in_radius(&range_query, radius);
    info!(
        "Points in Range for ({}, {}, {}, r={}):",
        range_query.x, range_query.y, range_query.z, radius
    );
    for point in results {
        info!(
            "({}, {}, {}) -> {:?}",
            point.x, point.y, point.z, point.data
        );
    }

    // Visualize the tree structure in the console.
    info!("Visualizing the octree");
    tree.visualize(0);

    // Optionally, generate a DOT file for Graphviz.
    tree.visualize_dot("octree.dot");
}

fn test_kdtree() {
    println!("{}", "=".repeat(100));
    println!("KdTree Example");

    // Example for 2D points:
    let mut tree2d = KdTree::<Point2D<&str>>::new(2);
    tree2d.insert(Point2D::new(1.0, 2.0, Some("A")));
    tree2d.insert(Point2D::new(3.0, 4.0, Some("B")));
    tree2d.insert(Point2D::new(5.0, 6.0, Some("C")));
    tree2d.insert(Point2D::new(7.0, 8.0, Some("D")));
    tree2d.insert(Point2D::new(9.0, 10.0, Some("E")));
    let nearest = tree2d.find_closest(&Point2D::new(2.0, 3.0, None), 1);
    println!("Nearest 2D point: {:?}", nearest);

    // Example for 3D points:
    let mut tree3d = KdTree::<Point3D<&str>>::new(3);
    tree3d.insert(Point3D::new(1.0, 2.0, 3.0, Some("A")));
    tree3d.insert(Point3D::new(4.0, 5.0, 6.0, Some("B")));
    tree3d.insert(Point3D::new(7.0, 8.0, 9.0, Some("C")));
    tree3d.insert(Point3D::new(10.0, 11.0, 12.0, Some("D")));
    tree3d.insert(Point3D::new(13.0, 14.0, 15.0, Some("E")));
    tree3d.insert(Point3D::new(16.0, 17.0, 18.0, Some("F")));
    let neighbors = tree3d.find_in_radius(&Point3D::new(2.0, 3.0, 4.0, None), 5.0);
    println!("3D points within radius: {:?}", neighbors);
}

fn test_rtree() {
    println!("{}", "=".repeat(100));
    println!("R-Tree Example");

    // Create an RTree with a maximum of 4 entries per node.
    let mut tree = RTree::<Point2D<&str>>::new(4);

    tree.insert(Point2D::new(1.0, 2.0, Some("A")));
    tree.insert(Point2D::new(3.0, 4.0, Some("B")));
    tree.insert(Point2D::new(2.0, 3.0, Some("C")));
    tree.insert(Point2D::new(5.0, 6.0, Some("D")));
    tree.insert(Point2D::new(7.0, 8.0, Some("E")));

    // Search for points in a query rectangle.
    let query = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 4.0,
        height: 4.0,
    };
    let found = tree.search(&query);
    println!("Found points: {:?}", found);

    // Create an RTree with a maximum of 4 entries per node for 3D points.
    let mut tree3d = RTree::<Point3D<&str>>::new(4);

    tree3d.insert(Point3D::new(1.0, 2.0, 3.0, Some("A")));
    tree3d.insert(Point3D::new(4.0, 5.0, 6.0, Some("B")));
    tree3d.insert(Point3D::new(7.0, 8.0, 9.0, Some("C")));
    tree3d.insert(Point3D::new(10.0, 11.0, 12.0, Some("D")));
    tree3d.insert(Point3D::new(13.0, 14.0, 15.0, Some("E")));

    // Search for points in a query rectangle.
    let query3d = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 4.0,
        height: 4.0,
        depth: 4.0,
    };

    let found3d = tree3d.search(&query3d);
    println!("Found points: {:?}", found3d);
}
