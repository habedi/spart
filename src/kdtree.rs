use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use tracing::info;

/// A trait that abstracts a point’s coordinate access and distance calculation.
/// This trait is used by the KD–tree.
pub trait KdPoint: Clone + PartialEq + std::fmt::Debug {
    /// Returns the number of dimensions (for example, 2 for Point2D, 3 for Point3D).
    fn dims(&self) -> usize;

    /// Returns the coordinate value for the given axis (0-indexed).
    fn coord(&self, axis: usize) -> f64;

    /// Returns the squared Euclidean distance between this point and another.
    fn distance_sq(&self, other: &Self) -> f64;
}

// -------------------------------------------------------------------
// KdPoint implementations for your geometry types
// (Assumes your geometry module defines Point2D and Point3D with f64 coordinates.)
// -------------------------------------------------------------------

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

// -------------------------------------------------------------------
// Helper struct for k–NN search so we don’t require KdPoint to be Ord.
// Only the distance is used for ordering.
// -------------------------------------------------------------------

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

// -------------------------------------------------------------------
// KD–tree implementation
// -------------------------------------------------------------------

/// A node in the KD–tree.
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

/// A KD–tree for points of type `P`.
#[derive(Debug)]
pub struct KdTree<P: KdPoint> {
    root: Option<Box<KdNode<P>>>,
    /// The dimensionality of the space (e.g., 2 or 3).
    k: usize,
}

impl<P: KdPoint> KdTree<P> {
    /// Creates a new, empty KD–tree for points in `k` dimensions.
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "Dimension must be greater than zero.");
        KdTree { root: None, k }
    }

    /// Inserts a point into the KD–tree.
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

    /// Recursive helper for insertion.
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

    /// Performs a k–nearest–neighbors search and returns up to `k_neighbors` closest points.
    pub fn find_closest(&self, target: &P, k_neighbors: usize) -> Vec<P> {
        info!(
            "Performing k-NN search for target {:?} with k={}",
            target, k_neighbors
        );
        let mut heap: BinaryHeap<HeapItem<P>> = BinaryHeap::new();
        Self::knn_search(&self.root, target, k_neighbors, 0, &mut heap);

        let mut result: Vec<(f64, P)> = heap
            .into_iter()
            .map(|item| (item.dist.into_inner(), item.point))
            .collect();

        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        result.into_iter().map(|(_d, p)| p).collect()
    }

    /// Recursive helper for k–NN search.
    fn knn_search(
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

            // Search the subtree on the same side as the target first.
            let (first, second) = if target_coord < node_coord {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };

            Self::knn_search(first, target, k_neighbors, depth + 1, heap);

            let diff = (target_coord - node_coord).abs();
            let diff_sq = diff * diff;
            if heap.len() < k_neighbors || diff_sq < heap.peek().unwrap().dist.into_inner() {
                Self::knn_search(second, target, k_neighbors, depth + 1, heap);
            }
        }
    }

    /// Finds all points within the given radius of `center`.
    pub fn find_in_radius(&self, center: &P, radius: f64) -> Vec<P> {
        info!("Finding points within radius {} of {:?}", radius, center);
        let mut found = Vec::new();
        let radius_sq = radius * radius;
        Self::range_search(&self.root, center, radius_sq, 0, radius, &mut found);
        found
    }

    /// Recursive helper for the range search.
    fn range_search(
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
                Self::range_search(&n.left, center, radius_sq, depth + 1, radius, found);
            }
            if center_coord + radius >= node_coord {
                Self::range_search(&n.right, center, radius_sq, depth + 1, radius, found);
            }
        }
    }
}
