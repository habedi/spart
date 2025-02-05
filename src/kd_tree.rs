use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::info;

pub trait KdPoint: Clone + PartialEq + std::fmt::Debug {
    fn dims(&self) -> usize;
    fn coord(&self, axis: usize) -> f64;
    fn distance_sq(&self, other: &Self) -> f64;
}

impl<T> KdPoint for crate::geometry::Point2D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    fn dims(&self) -> usize {
        2
    }
    fn coord(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            _ => panic!("Point2D has only 2 dimensions; axis {} is invalid", axis),
        }
    }
    fn distance_sq(&self, other: &Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl<T> KdPoint for crate::geometry::Point3D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    fn dims(&self) -> usize {
        3
    }
    fn coord(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Point3D has only 3 dimensions; axis {} is invalid", axis),
        }
    }
    fn distance_sq(&self, other: &Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }
}

#[derive(Debug)]
struct HeapItem<P> {
    dist: OrderedFloat<f64>,
    point: P,
}

impl<P> PartialEq for HeapItem<P> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl<P> Eq for HeapItem<P> {}

impl<P> PartialOrd for HeapItem<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<P> Ord for HeapItem<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

#[derive(Debug)]
struct KdNode<P: KdPoint> {
    point: P,
    left: Option<Box<KdNode<P>>>,
    right: Option<Box<KdNode<P>>>,
}

impl<P: KdPoint> KdNode<P> {
    fn new(point: P) -> Self {
        KdNode {
            point,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct KdTree<P: KdPoint> {
    root: Option<Box<KdNode<P>>>,
    k: usize,
}

impl<P: KdPoint> KdTree<P> {
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "Dimension must be greater than zero.");
        KdTree { root: None, k }
    }

    pub fn insert(&mut self, point: P) {
        assert!(
            point.dims() == self.k,
            "Point dimension {} does not match KDTree dimension {}",
            point.dims(),
            self.k
        );
        info!("Inserting point: {:?}", point);
        self.root = Some(Self::insert_rec(self.root.take(), point, 0, self.k));
    }

    fn insert_rec(
        node: Option<Box<KdNode<P>>>,
        point: P,
        depth: usize,
        k: usize,
    ) -> Box<KdNode<P>> {
        if let Some(mut current) = node {
            let axis = depth % k;
            if point.coord(axis) < current.point.coord(axis) {
                current.left = Some(Self::insert_rec(current.left.take(), point, depth + 1, k));
            } else {
                current.right = Some(Self::insert_rec(current.right.take(), point, depth + 1, k));
            }
            current
        } else {
            Box::new(KdNode::new(point))
        }
    }

    pub fn knn_search(&self, target: &P, k_neighbors: usize) -> Vec<P> {
        info!(
            "Performing k-NN search for target {:?} with k={}",
            target, k_neighbors
        );
        let mut heap: BinaryHeap<HeapItem<P>> = BinaryHeap::new();
        Self::knn_search_rec(&self.root, target, k_neighbors, 0, &mut heap);
        let mut result: Vec<(f64, P)> = heap
            .into_iter()
            .map(|item| (item.dist.into_inner(), item.point))
            .collect();
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        result.into_iter().map(|(_d, p)| p).collect()
    }

    fn knn_search_rec(
        node: &Option<Box<KdNode<P>>>,
        target: &P,
        k_neighbors: usize,
        depth: usize,
        heap: &mut BinaryHeap<HeapItem<P>>,
    ) {
        if let Some(ref n) = node {
            let dist_sq = target.distance_sq(&n.point);
            let dist = OrderedFloat(dist_sq);
            if heap.len() < k_neighbors {
                heap.push(HeapItem {
                    dist,
                    point: n.point.clone(),
                });
            } else if let Some(top) = heap.peek() {
                if dist < top.dist {
                    heap.pop();
                    heap.push(HeapItem {
                        dist,
                        point: n.point.clone(),
                    });
                }
            }
            let axis = depth % target.dims();
            let target_coord = target.coord(axis);
            let node_coord = n.point.coord(axis);
            let (first, second) = if target_coord < node_coord {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };
            Self::knn_search_rec(first, target, k_neighbors, depth + 1, heap);
            let diff = (target_coord - node_coord).abs();
            let diff_sq = diff * diff;
            if heap.len() < k_neighbors || diff_sq < heap.peek().unwrap().dist.into_inner() {
                Self::knn_search_rec(second, target, k_neighbors, depth + 1, heap);
            }
        }
    }

    pub fn range_search(&self, center: &P, radius: f64) -> Vec<P> {
        info!("Finding points within radius {} of {:?}", radius, center);
        let mut found = Vec::new();
        let radius_sq = radius * radius;
        Self::range_search_rec(&self.root, center, radius_sq, 0, radius, &mut found);
        found
    }

    fn range_search_rec(
        node: &Option<Box<KdNode<P>>>,
        center: &P,
        radius_sq: f64,
        depth: usize,
        radius: f64,
        found: &mut Vec<P>,
    ) {
        if let Some(ref n) = node {
            let dist_sq = center.distance_sq(&n.point);
            if dist_sq <= radius_sq {
                found.push(n.point.clone());
            }
            let axis = depth % center.dims();
            let center_coord = center.coord(axis);
            let node_coord = n.point.coord(axis);
            if center_coord - radius <= node_coord {
                Self::range_search_rec(&n.left, center, radius_sq, depth + 1, radius, found);
            }
            if center_coord + radius >= node_coord {
                Self::range_search_rec(&n.right, center, radius_sq, depth + 1, radius, found);
            }
        }
    }

    pub fn delete(&mut self, point: &P) -> bool {
        info!("Attempting to delete point: {:?}", point);
        let mut points = Vec::new();
        Self::collect_points(&self.root, &mut points);
        let initial_count = points.len();
        points.retain(|p| p != point);
        if points.len() == initial_count {
            info!("Point not found; nothing deleted.");
            return false;
        }
        self.root = None;
        for p in points {
            self.insert(p);
        }
        info!("Point deleted and tree rebuilt.");
        true
    }

    fn collect_points(node: &Option<Box<KdNode<P>>>, points: &mut Vec<P>) {
        if let Some(ref n) = node {
            points.push(n.point.clone());
            Self::collect_points(&n.left, points);
            Self::collect_points(&n.right, points);
        }
    }
}
