use crate::geometry::{Cube, HeapItem, Point3D};
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::{debug, info};

pub struct Octree<T: Clone + PartialEq> {
    boundary: Cube,
    points: Vec<Point3D<T>>,
    capacity: usize,
    divided: bool,
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
        self.front_top_left = Some(Box::new(Octree::new(
            &Cube {
                x,
                y,
                z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.front_top_right = Some(Box::new(Octree::new(
            &Cube {
                x: x + w,
                y,
                z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.front_bottom_left = Some(Box::new(Octree::new(
            &Cube {
                x,
                y: y + h,
                z,
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
                z,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_top_left = Some(Box::new(Octree::new(
            &Cube {
                x,
                y,
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
                y,
                z: z + d,
                width: w,
                height: h,
                depth: d,
            },
            self.capacity,
        )));
        self.back_bottom_left = Some(Box::new(Octree::new(
            &Cube {
                x,
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
        let points = std::mem::take(&mut self.points);
        for point in points {
            self.insert(point);
        }
    }

    fn children_mut(&mut self) -> Vec<&mut Octree<T>> {
        let mut children = Vec::with_capacity(8);
        if let Some(ref mut child) = self.front_top_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_top_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_bottom_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_bottom_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_top_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_top_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_bottom_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_bottom_right {
            children.push(child.as_mut());
        }
        children
    }

    fn children(&self) -> Vec<&Octree<T>> {
        let mut children = Vec::with_capacity(8);
        if let Some(ref child) = self.front_top_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_top_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_bottom_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_bottom_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_top_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_top_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_bottom_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_bottom_right {
            children.push(child.as_ref());
        }
        children
    }

    fn min_distance_sq(&self, target: &Point3D<T>) -> f64 {
        let tx = target.x;
        let ty = target.y;
        let tz = target.z;
        let cx = self.boundary.x;
        let cy = self.boundary.y;
        let cz = self.boundary.z;
        let cw = self.boundary.width;
        let ch = self.boundary.height;
        let cd = self.boundary.depth;
        let dx = if tx < cx {
            cx - tx
        } else if tx > cx + cw {
            tx - (cx + cw)
        } else {
            0.0
        };
        let dy = if ty < cy {
            cy - ty
        } else if ty > cy + ch {
            ty - (cy + ch)
        } else {
            0.0
        };
        let dz = if tz < cz {
            cz - tz
        } else if tz > cz + cd {
            tz - (cz + cd)
        } else {
            0.0
        };
        dx * dx + dy * dy + dz * dz
    }

    pub fn insert(&mut self, point: Point3D<T>) -> bool {
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }
        if self.divided {
            let children = self.children_mut();
            let n = children.len();
            for (i, child) in children.into_iter().enumerate() {
                if i < n - 1 {
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
            info!("Inserting point {:?} into Octree", point);
            self.points.push(point);
            return true;
        }
        self.subdivide();
        self.insert(point)
    }

    pub fn knn_search(&self, target: &Point3D<T>, k: usize) -> Vec<Point3D<T>> {
        let mut heap: BinaryHeap<HeapItem<T>> = BinaryHeap::new();
        self.knn_search_helper(target, k, &mut heap);
        let mut result: Vec<Point3D<T>> = heap
            .into_sorted_vec()
            .into_iter()
            .filter_map(|item| item.point_3d)
            .collect();
        result.reverse();
        result
    }

    fn knn_search_helper(&self, target: &Point3D<T>, k: usize, heap: &mut BinaryHeap<HeapItem<T>>) {
        for point in &self.points {
            let dist_sq = point.distance_sq(target);
            let item = HeapItem {
                neg_distance: OrderedFloat(-dist_sq),
                point_2d: None,
                point_3d: Some(point.clone()),
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

    pub fn range_search(&self, center: &Point3D<T>, radius: f64) -> Vec<Point3D<T>> {
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

    pub fn delete(&mut self, point: &Point3D<T>) -> bool {
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
            info!("Deleting point {:?} from Octree", point);
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
                if let Some(child) = self.front_top_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_top_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_bottom_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_bottom_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_top_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_top_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_bottom_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_bottom_right.take() {
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
