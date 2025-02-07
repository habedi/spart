use crate::geometry::{HeapItem, Point2D, Rectangle};
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct Quadtree<T: Clone + PartialEq> {
    boundary: Rectangle,
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
        let old_points = std::mem::take(&mut self.points);
        for point in old_points {
            let inserted = self.insert(point);
            if !inserted {
                debug!("Failed to reinsert point during subdivision");
            }
        }
    }

    pub fn insert(&mut self, point: Point2D<T>) -> bool {
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }
        if self.divided {
            let children = self.children_mut();
            let num_children = children.len();
            for (i, child) in children.into_iter().enumerate() {
                if i < num_children - 1 {
                    if child.insert(point.clone()) {
                        return true;
                    }
                } else {
                    return child.insert(point);
                }
            }
            return false;
        }
        if self.points.len() < self.capacity {
            info!("Inserting point {:?} into Quadtree", point);
            self.points.push(point);
            return true;
        }
        self.subdivide();
        self.insert(point)
    }

    fn children_mut(&mut self) -> Vec<&mut Quadtree<T>> {
        let mut children = Vec::with_capacity(4);
        if let Some(ref mut child) = self.northeast {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.northwest {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.southeast {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.southwest {
            children.push(child.as_mut());
        }
        children
    }

    fn children(&self) -> Vec<&Quadtree<T>> {
        let mut children = Vec::with_capacity(4);
        if let Some(ref child) = self.northeast {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.northwest {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.southeast {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.southwest {
            children.push(child.as_ref());
        }
        children
    }

    fn min_distance_sq(&self, target: &Point2D<T>) -> f64 {
        let mut dx = 0.0;
        if target.x < self.boundary.x {
            dx = self.boundary.x - target.x;
        } else if target.x > self.boundary.x + self.boundary.width {
            dx = target.x - (self.boundary.x + self.boundary.width);
        }
        let mut dy = 0.0;
        if target.y < self.boundary.y {
            dy = self.boundary.y - target.y;
        } else if target.y > self.boundary.y + self.boundary.height {
            dy = target.y - (self.boundary.y + self.boundary.height);
        }
        dx * dx + dy * dy
    }

    pub fn knn_search(&self, target: &Point2D<T>, k: usize) -> Vec<Point2D<T>> {
        let mut heap: BinaryHeap<HeapItem<T>> = BinaryHeap::new();
        self.knn_search_helper(target, k, &mut heap);
        let mut results: Vec<Point2D<T>> = heap
            .into_sorted_vec()
            .into_iter()
            .filter_map(|item| item.point_2d)
            .collect();
        results.reverse();
        results
    }

    fn knn_search_helper(&self, target: &Point2D<T>, k: usize, heap: &mut BinaryHeap<HeapItem<T>>) {
        for point in &self.points {
            let dist_sq = point.distance_sq(target);
            let item = HeapItem {
                neg_distance: OrderedFloat(-dist_sq),
                point_2d: Some(point.clone()),
                point_3d: None,
            };
            heap.push(item);
            if heap.len() > k {
                heap.pop();
            }
        }
        if self.divided {
            for child in self.children() {
                if heap.len() == k {
                    let current_farthest = -heap.peek().unwrap().neg_distance.into_inner();
                    if child.min_distance_sq(target) > current_farthest {
                        continue;
                    }
                }
                child.knn_search_helper(target, k, heap);
            }
        }
    }

    pub fn range_search(&self, center: &Point2D<T>, radius: f64) -> Vec<Point2D<T>> {
        let mut found = Vec::new();
        let radius_sq = radius * radius;
        if self.min_distance_sq(center) > radius_sq {
            return found;
        }
        for point in &self.points {
            if point.distance_sq(center) <= radius_sq {
                found.push(point.clone());
            }
        }
        if self.divided {
            for child in self.children() {
                found.extend(child.range_search(center, radius));
            }
        }
        found
    }

    pub fn delete(&mut self, point: &Point2D<T>) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }
        let mut deleted = false;
        if self.divided {
            for child in self.children_mut() {
                if child.delete(point) {
                    deleted = true;
                }
            }
            self.try_merge();
            return deleted;
        }
        if let Some(pos) = self.points.iter().position(|p| p == point) {
            info!("Deleting point {:?} from Quadtree", point);
            self.points.remove(pos);
            return true;
        }
        false
    }

    fn try_merge(&mut self) {
        if !self.divided {
            return;
        }
        for child in self.children_mut() {
            child.try_merge();
        }
        let children = self.children();
        if children.iter().all(|child| !child.divided) {
            let total_points: usize = children.iter().map(|child| child.points.len()).sum();
            if total_points <= self.capacity {
                let mut merged_points = Vec::with_capacity(total_points);
                if let Some(child) = self.northeast.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.northwest.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.southeast.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.southwest.take() {
                    merged_points.extend(child.points);
                }
                info!(
                    "Merging children into parent node at boundary {:?} with {} points",
                    self.boundary,
                    merged_points.len()
                );
                self.points.extend(merged_points);
                self.divided = false;
            }
        }
    }
}
