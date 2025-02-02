use crate::geometry::{Point2D, Rectangle};
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct Quadtree<T: Clone + PartialEq> {
    boundary: Rectangle,
    /// Points are stored **only in leaf nodes**.
    points: Vec<Point2D<T>>,
    capacity: usize,
    divided: bool,
    northeast: Option<Box<Quadtree<T>>>,
    northwest: Option<Box<Quadtree<T>>>,
    southeast: Option<Box<Quadtree<T>>>,
    southwest: Option<Box<Quadtree<T>>>,
}

impl<T: Clone + PartialEq + std::fmt::Debug> Quadtree<T> {
    pub fn new(boundary: &Rectangle, capacity: usize) -> Self {
        info!(
            "Creating new Quadtree with boundary: {:?} and capacity: {}",
            boundary, capacity
        );
        Quadtree {
            boundary: boundary.clone(),
            points: Vec::new(),
            capacity,
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    pub fn subdivide(&mut self) {
        info!("Subdividing Quadtree at boundary: {:?}", self.boundary);
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        self.northeast = Some(Box::new(Quadtree::new(
            &Rectangle {
                x: x + w,
                y,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.northwest = Some(Box::new(Quadtree::new(
            &Rectangle {
                x,
                y,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.southeast = Some(Box::new(Quadtree::new(
            &Rectangle {
                x: x + w,
                y: y + h,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.southwest = Some(Box::new(Quadtree::new(
            &Rectangle {
                x,
                y: y + h,
                width: w,
                height: h,
            },
            self.capacity,
        )));

        self.divided = true;

        // Move existing points down into the appropriate children.
        let points = std::mem::take(&mut self.points);
        for point in points {
            // Since self.divided is now true, the insert method will
            // delegate the insertion to the children.
            self.insert(point);
        }
    }

    pub fn insert(&mut self, point: Point2D<T>) -> bool {
        // If the point is not in this node's boundary, bail early.
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }

        // IMPORTANT: Once subdivided, this node is an internal node and must not store points.
        if self.divided {
            return self
                .northeast
                .as_mut()
                .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .northwest
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self
                    .southeast
                    .as_mut()
                    .map_or(false, |qt| qt.insert(point.clone()))
                || self.southwest.as_mut().map_or(false, |qt| qt.insert(point));
        }

        // Otherwise, if we are still a leaf node, insert the point if there is capacity.
        if self.points.len() < self.capacity {
            info!("Inserting point {:?} into Quadtree", point);
            self.points.push(point);
            return true;
        }

        // No capacity left in this leaf node: subdivide and reinsert the point.
        self.subdivide();
        self.insert(point)
    }

    pub fn find_closest(&self, target: &Point2D<T>, k: usize) -> Vec<Point2D<T>> {
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
            if let Some(ne) = &self.northeast {
                points_vec.extend(ne.find_closest(target, k));
            }
            if let Some(nw) = &self.northwest {
                points_vec.extend(nw.find_closest(target, k));
            }
            if let Some(se) = &self.southeast {
                points_vec.extend(se.find_closest(target, k));
            }
            if let Some(sw) = &self.southwest {
                points_vec.extend(sw.find_closest(target, k));
            }
        }

        heap.into_sorted_vec()
            .into_iter()
            .map(|(_, idx)| points_vec[idx].clone())
            .collect()
    }

    pub fn find_in_radius(&self, center: &Point2D<T>, radius: f64) -> Vec<Point2D<T>> {
        info!("Finding points within radius {} of {:?}", radius, center);
        let mut found = Vec::new();
        let radius_sq = radius * radius;

        for point in &self.points {
            if point.distance_sq(center) <= radius_sq {
                found.push(point.clone());
            }
        }

        if self.divided {
            if let Some(ne) = &self.northeast {
                found.extend(ne.find_in_radius(center, radius));
            }
            if let Some(nw) = &self.northwest {
                found.extend(nw.find_in_radius(center, radius));
            }
            if let Some(se) = &self.southeast {
                found.extend(se.find_in_radius(center, radius));
            }
            if let Some(sw) = &self.southwest {
                found.extend(sw.find_in_radius(center, radius));
            }
        }

        found
    }

    pub fn visualize(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!(
            "{}+ Quadtree ([Quad: x:{:.1}, y:{:.1}, w:{:.1}, h:{:.1}], {} points)",
            indent,
            self.boundary.x,
            self.boundary.y,
            self.boundary.width,
            self.boundary.height,
            self.points.len()
        );

        for point in &self.points {
            println!(
                "{}  - Point2D ([x:{:.1}, y:{:.1}], data:{:?})",
                indent, point.x, point.y, point.data
            );
        }

        if self.divided {
            if let Some(ne) = &self.northeast {
                ne.visualize(depth + 1);
            }
            if let Some(nw) = &self.northwest {
                nw.visualize(depth + 1);
            }
            if let Some(se) = &self.southeast {
                se.visualize(depth + 1);
            }
            if let Some(sw) = &self.southwest {
                sw.visualize(depth + 1);
            }
        }
    }

    pub fn visualize_dot(&self, filename: &str) {
        let mut graph = String::new();
        graph.push_str("digraph Quadtree {\n");
        self.visualize_node(&mut graph, 0);
        graph.push_str("}\n");

        std::fs::write(filename, graph).expect("Unable to write file");
    }

    fn visualize_node(&self, graph: &mut String, id: usize) -> usize {
        let mut current_id = id;
        // Use shape=box to force a rectangle, style=rounded for rounded corners,
        // and optionally style=filled with a fillcolor for a nicer look.
        graph.push_str(&format!(
            "  node{} [shape=box, style=rounded, fillcolor=lightblue, label=\"[({}, {}, {}, {}), {}]\"];\n",
            current_id, self.boundary.x, self.boundary.y, self.boundary.width, self.boundary.height, self.points.len()
        ));

        if self.divided {
            if let Some(ne) = &self.northeast {
                current_id += 1;
                let ne_id = current_id;
                graph.push_str(&format!("  node{} -> node{};\n", id, ne_id));
                current_id = ne.visualize_node(graph, ne_id);
            }
            if let Some(nw) = &self.northwest {
                current_id += 1;
                let nw_id = current_id;
                graph.push_str(&format!("  node{} -> node{};\n", id, nw_id));
                current_id = nw.visualize_node(graph, nw_id);
            }
            if let Some(se) = &self.southeast {
                current_id += 1;
                let se_id = current_id;
                graph.push_str(&format!("  node{} -> node{};\n", id, se_id));
                current_id = se.visualize_node(graph, se_id);
            }
            if let Some(sw) = &self.southwest {
                current_id += 1;
                let sw_id = current_id;
                graph.push_str(&format!("  node{} -> node{};\n", id, sw_id));
                current_id = sw.visualize_node(graph, sw_id);
            }
        }

        current_id
    }
}
