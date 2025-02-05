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
        let points = std::mem::take(&mut self.points);
        for point in points {
            self.insert(point);
        }
    }

    pub fn insert(&mut self, point: Point2D<T>) -> bool {
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }
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
        if self.points.len() < self.capacity {
            info!("Inserting point {:?} into Quadtree", point);
            self.points.push(point);
            return true;
        }
        self.subdivide();
        self.insert(point)
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
                point_2d: Option::from(point.clone()),
                point_3d: None,
            };
            heap.push(item);
            if heap.len() > k {
                heap.pop();
            }
        }
        if self.divided {
            if let Some(ref ne) = self.northeast {
                ne.knn_search_helper(target, k, heap);
            }
            if let Some(ref nw) = self.northwest {
                nw.knn_search_helper(target, k, heap);
            }
            if let Some(ref se) = self.southeast {
                se.knn_search_helper(target, k, heap);
            }
            if let Some(ref sw) = self.southwest {
                sw.knn_search_helper(target, k, heap);
            }
        }
    }

    pub fn range_search(&self, center: &Point2D<T>, radius: f64) -> Vec<Point2D<T>> {
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
                found.extend(ne.range_search(center, radius));
            }
            if let Some(nw) = &self.northwest {
                found.extend(nw.range_search(center, radius));
            }
            if let Some(se) = &self.southeast {
                found.extend(se.range_search(center, radius));
            }
            if let Some(sw) = &self.southwest {
                found.extend(sw.range_search(center, radius));
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
            if let Some(ref mut child) = self.northeast {
                deleted |= child.delete(point);
            }
            if let Some(ref mut child) = self.northwest {
                deleted |= child.delete(point);
            }
            if let Some(ref mut child) = self.southeast {
                deleted |= child.delete(point);
            }
            if let Some(ref mut child) = self.southwest {
                deleted |= child.delete(point);
            }
            self.try_merge();
            return deleted;
        } else {
            if let Some(pos) = self.points.iter().position(|p| p == point) {
                info!("Deleting point {:?} from Quadtree", point);
                self.points.remove(pos);
                return true;
            }
        }
        false
    }

    fn try_merge(&mut self) {
        if !self.divided {
            return;
        }
        if let Some(ref mut ne) = self.northeast {
            ne.try_merge();
        }
        if let Some(ref mut nw) = self.northwest {
            nw.try_merge();
        }
        if let Some(ref mut se) = self.southeast {
            se.try_merge();
        }
        if let Some(ref mut sw) = self.southwest {
            sw.try_merge();
        }
        let merge_possible = self
            .northeast
            .as_ref()
            .map(|child| !child.divided)
            .unwrap_or(true)
            && self
                .northwest
                .as_ref()
                .map(|child| !child.divided)
                .unwrap_or(true)
            && self
                .southeast
                .as_ref()
                .map(|child| !child.divided)
                .unwrap_or(true)
            && self
                .southwest
                .as_ref()
                .map(|child| !child.divided)
                .unwrap_or(true);
        if merge_possible {
            let total_points = self
                .northeast
                .as_ref()
                .map(|child| child.points.len())
                .unwrap_or(0)
                + self
                    .northwest
                    .as_ref()
                    .map(|child| child.points.len())
                    .unwrap_or(0)
                + self
                    .southeast
                    .as_ref()
                    .map(|child| child.points.len())
                    .unwrap_or(0)
                + self
                    .southwest
                    .as_ref()
                    .map(|child| child.points.len())
                    .unwrap_or(0);
            if total_points <= self.capacity {
                let mut merged_points = Vec::new();
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
                self.points = merged_points;
                self.divided = false;
            }
        }
    }
}
