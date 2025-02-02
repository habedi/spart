use crate::geometry::{Cube, Point3D};
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct Octree<T: Clone + PartialEq> {
    boundary: Cube,
    /// Points are stored **only in leaf nodes**.
    points: Vec<Point3D<T>>,
    capacity: usize,
    divided: bool,
    // Eight children representing the eight octants:
    front_top_left: Option<Box<Octree<T>>>,
    front_top_right: Option<Box<Octree<T>>>,
    front_bottom_left: Option<Box<Octree<T>>>,
    front_bottom_right: Option<Box<Octree<T>>>,
    back_top_left: Option<Box<Octree<T>>>,
    back_top_right: Option<Box<Octree<T>>>,
    back_bottom_left: Option<Box<Octree<T>>>,
    back_bottom_right: Option<Box<Octree<T>>>,
}

impl<T: Clone + PartialEq + std::fmt::Debug> Octree<T> {
    pub fn new(boundary: &Cube, capacity: usize) -> Self {
        info!(
            "Creating new Octree with boundary: {:?} and capacity: {}",
            boundary, capacity
        );
        Octree {
            boundary: boundary.clone(),
            points: Vec::new(),
            capacity,
            divided: false,
            front_top_left: None,
            front_top_right: None,
            front_bottom_left: None,
            front_bottom_right: None,
            back_top_left: None,
            back_top_right: None,
            back_bottom_left: None,
            back_bottom_right: None,
        }
    }

    pub fn subdivide(&mut self) {
        info!("Subdividing Octree at boundary: {:?}", self.boundary);
        let x = self.boundary.x;
        let y = self.boundary.y;
        let z = self.boundary.z;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;
        let d = self.boundary.depth / 2.0;

        // Create eight child octants.
        self.front_top_left = Some(Box::new(Octree::new(
            &Cube {
                x: x,
                y: y,
                z: z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.front_top_right = Some(Box::new(Octree::new(
            &Cube {
                x: x + w,
                y: y,
                z: z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.front_bottom_left = Some(Box::new(Octree::new(
            &Cube {
                x: x,
                y: y + h,
                z: z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.front_bottom_right = Some(Box::new(Octree::new(
            &Cube {
                x: x + w,
                y: y + h,
                z: z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_top_left = Some(Box::new(Octree::new(
            &Cube {
                x: x,
                y: y,
                z: z + d,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_top_right = Some(Box::new(Octree::new(
            &Cube {
                x: x + w,
                y: y,
                z: z + d,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_bottom_left = Some(Box::new(Octree::new(
            &Cube {
                x: x,
                y: y + h,
                z: z + d,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_bottom_right = Some(Box::new(Octree::new(
            &Cube {
                x: x + w,
                y: y + h,
                z: z + d,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));

        self.divided = true;

        // Move existing points down into the appropriate children.
        let points = std::mem::take(&mut self.points);
        for point in points {
            self.insert(point);
        }
    }

    pub fn insert(&mut self, point: Point3D<T>) -> bool {
        // If the point is not inside this node’s boundary, return false.
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }

        // If already subdivided, delegate the insertion to the children.
        if self.divided {
            return self
                .front_top_left
                .as_mut()
                .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .front_top_right
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .front_bottom_left
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .front_bottom_right
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .back_top_left
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .back_top_right
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .back_bottom_left
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .back_bottom_right
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point));
        }

        // If there’s room in this leaf, store the point here.
        if self.points.len() < self.capacity {
            info!("Inserting point {:?} into Octree", point);
            self.points.push(point);
            return true;
        }

        // Otherwise, subdivide and then insert.
        self.subdivide();
        self.insert(point)
    }

    pub fn find_closest(&self, target: &Point3D<T>, k: usize) -> Vec<Point3D<T>> {
        info!("Performing KNN search for target {:?} with k={}", target, k);
        let mut heap = BinaryHeap::new();
        let mut points_vec = Vec::new();

        for point in &self.points {
            let dist = OrderedFloat(-point.distance_sq(target));
            points_vec.push(point.clone());
            heap.push((dist, points_vec.len() - 1));

            if heap.len() > k {
                heap.pop();
            }
        }

        if self.divided {
            // Recursively search in each child.
            if let Some(child) = &self.front_top_left {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.front_top_right {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.front_bottom_left {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.front_bottom_right {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.back_top_left {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.back_top_right {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.back_bottom_left {
                points_vec.extend(child.find_closest(target, k));
            }
            if let Some(child) = &self.back_bottom_right {
                points_vec.extend(child.find_closest(target, k));
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|(_, idx)| points_vec[idx].clone())
            .collect()
    }

    pub fn find_in_radius(&self, center: &Point3D<T>, radius: f64) -> Vec<Point3D<T>> {
        info!("Finding points within radius {} of {:?}", radius, center);
        let mut found = Vec::new();
        let radius_sq = radius * radius;

        for point in &self.points {
            if point.distance_sq(center) <= radius_sq {
                found.push(point.clone());
            }
        }

        if self.divided {
            if let Some(child) = &self.front_top_left {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.front_top_right {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.front_bottom_left {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.front_bottom_right {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.back_top_left {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.back_top_right {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.back_bottom_left {
                found.extend(child.find_in_radius(center, radius));
            }
            if let Some(child) = &self.back_bottom_right {
                found.extend(child.find_in_radius(center, radius));
            }
        }

        found
    }

    pub fn visualize(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!(
            "{}+ Octree ([Cube: x:{:.1}, y:{:.1}, z:{:.1}, w:{:.1}, h:{:.1}, d:{:.1}], {} points)",
            indent,
            self.boundary.x,
            self.boundary.y,
            self.boundary.z,
            self.boundary.width,
            self.boundary.height,
            self.boundary.depth,
            self.points.len()
        );

        for point in &self.points {
            println!(
                "{}  - Point3D ([x:{:.1}, y:{:.1}, z:{:.1}], data:{:?})",
                indent, point.x, point.y, point.z, point.data
            );
        }

        if self.divided {
            if let Some(child) = &self.front_top_left {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.front_top_right {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.front_bottom_left {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.front_bottom_right {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.back_top_left {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.back_top_right {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.back_bottom_left {
                child.visualize(depth + 1);
            }
            if let Some(child) = &self.back_bottom_right {
                child.visualize(depth + 1);
            }
        }
    }

    pub fn visualize_dot(&self, filename: &str) {
        let mut graph = String::new();
        graph.push_str("digraph Octree {\n");
        self.visualize_node(&mut graph, 0);
        graph.push_str("}\n");

        std::fs::write(filename, graph).expect("Unable to write file");
    }

    fn visualize_node(&self, graph: &mut String, id: usize) -> usize {
        let mut current_id = id;
        // Create a node label with cube bounds and point count.
        graph.push_str(&format!(
            "  node{} [shape=box, style=rounded, fillcolor=lightblue, label=\"[Cube: ({:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}), {}]\"];\n",
            current_id,
            self.boundary.x,
            self.boundary.y,
            self.boundary.z,
            self.boundary.width,
            self.boundary.height,
            self.boundary.depth,
            self.points.len()
        ));

        if self.divided {
            // For each child, create an edge and recursively visualize.
            for child_opt in [
                &self.front_top_left,
                &self.front_top_right,
                &self.front_bottom_left,
                &self.front_bottom_right,
                &self.back_top_left,
                &self.back_top_right,
                &self.back_bottom_left,
                &self.back_bottom_right,
            ]
            .iter()
            {
                if let Some(child) = child_opt {
                    current_id += 1;
                    let child_id = current_id;
                    graph.push_str(&format!("  node{} -> node{};\n", id, child_id));
                    current_id = child.visualize_node(graph, child_id);
                }
            }
        }

        current_id
    }
}
