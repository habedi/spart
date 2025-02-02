// src/rtree.rs

use crate::geometry::{Cube, Point2D, Point3D, Rectangle};

/// A trait for types that can serve as bounding volumes.
/// For 2D, this might be a Rectangle; for 3D, a Cube (or cuboid).
pub trait BoundingVolume: Clone {
    fn area(&self) -> f64; // For 3D, this means "volume"
    fn union(&self, other: &Self) -> Self;
    fn enlargement(&self, other: &Self) -> f64 {
        self.union(other).area() - self.area()
    }
    fn intersects(&self, other: &Self) -> bool;
}

// --- Implement BoundingVolume for Rectangle (2D) ---

impl BoundingVolume for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn union(&self, other: &Self) -> Self {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        Rectangle {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y)
    }
}

// --- Implement BoundingVolume for Cube (3D) ---
// (For simplicity, we treat Cube similarly to Rectangle but in 3D.)

impl BoundingVolume for Cube {
    fn area(&self) -> f64 {
        // Here "area" means volume.
        self.width * self.height * self.depth
    }
    fn union(&self, other: &Self) -> Self {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let z1 = self.z.min(other.z);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        let z2 = (self.z + self.depth).max(other.z + other.depth);
        Cube {
            x: x1,
            y: y1,
            z: z1,
            width: x2 - x1,
            height: y2 - y1,
            depth: z2 - z1,
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y
            || other.z > self.z + self.depth
            || other.z + other.depth < self.z)
    }
}

/// A trait for objects that can be stored in an R–tree.
/// The associated type `B` is the bounding volume type for that object.
pub trait RTreeObject: std::fmt::Debug {
    type B: BoundingVolume + std::fmt::Debug;
    fn mbr(&self) -> Self::B;
}

/// An entry in the R–tree is either a leaf entry (storing an object and its MBR)
/// or an internal (node) entry (storing a child node and that child’s MBR).
#[derive(Debug, Clone)]
pub enum RTreeEntry<T: RTreeObject> {
    Leaf { mbr: T::B, object: T },
    Node { mbr: T::B, child: Box<RTreeNode<T>> },
}

impl<T: RTreeObject> RTreeEntry<T> {
    /// Returns a reference to the entry's MBR.
    pub fn mbr(&self) -> &T::B {
        match self {
            RTreeEntry::Leaf { mbr, .. } => mbr,
            RTreeEntry::Node { mbr, .. } => mbr,
        }
    }
}

/// A node in the R–tree.
#[derive(Debug, Clone)]
pub struct RTreeNode<T: RTreeObject> {
    pub entries: Vec<RTreeEntry<T>>,
    pub is_leaf: bool,
}

/// The R–tree structure.
#[derive(Debug)]
pub struct RTree<T: RTreeObject> {
    root: RTreeNode<T>,
    max_entries: usize,
}

impl<T: RTreeObject> RTree<T> {
    /// Creates a new, empty R–tree.
    ///
    /// * `max_entries` is the maximum number of entries a node may hold before splitting.
    pub fn new(max_entries: usize) -> Self {
        RTree {
            root: RTreeNode {
                entries: Vec::new(),
                is_leaf: true,
            },
            max_entries,
        }
    }

    /// Inserts an object into the R–tree.
    pub fn insert(&mut self, object: T) {
        let entry = RTreeEntry::Leaf {
            mbr: object.mbr(),
            object,
        };
        insert_entry_node(&mut self.root, entry);
        if self.root.entries.len() > self.max_entries {
            self.split_root();
        }
    }

    /// Splits the root node when it overflows.
    fn split_root(&mut self) {
        let old_entries = std::mem::take(&mut self.root.entries);
        let (group1, group2) = split_entries(old_entries, self.max_entries);
        let child1 = RTreeNode {
            entries: group1,
            is_leaf: self.root.is_leaf,
        };
        let child2 = RTreeNode {
            entries: group2,
            is_leaf: self.root.is_leaf,
        };
        let mbr1 = compute_group_mbr(&child1.entries);
        let mbr2 = compute_group_mbr(&child2.entries);
        self.root.is_leaf = false;
        self.root.entries.push(RTreeEntry::Node {
            mbr: mbr1,
            child: Box::new(child1),
        });
        self.root.entries.push(RTreeEntry::Node {
            mbr: mbr2,
            child: Box::new(child2),
        });
    }

    /// Searches for all objects whose MBR intersects with the given query bounding volume.
    pub fn search(&self, query: &T::B) -> Vec<&T> {
        let mut result = Vec::new();
        search_node(&self.root, query, &mut result);
        result
    }
}

/// A standalone recursive helper to insert an entry into a node.
fn insert_entry_node<T: RTreeObject>(node: &mut RTreeNode<T>, entry: RTreeEntry<T>) {
    if node.is_leaf {
        node.entries.push(entry);
    } else {
        // Choose the child whose MBR requires the least enlargement to include the new entry.
        let mut best_index: Option<usize> = None;
        let mut best_enlargement = f64::INFINITY;
        for (i, child_entry) in node.entries.iter().enumerate() {
            if let RTreeEntry::Node { mbr, .. } = child_entry {
                let enlargement = mbr.enlargement(entry.mbr());
                if enlargement < best_enlargement {
                    best_enlargement = enlargement;
                    best_index = Some(i);
                } else if (enlargement - best_enlargement).abs() < std::f64::EPSILON {
                    // Tie-breaker: choose the one with the smaller area.
                    if let Some(current_best) = best_index {
                        if mbr.area() < node.entries[current_best].mbr().area() {
                            best_index = Some(i);
                        }
                    }
                }
            }
        }
        if let Some(best_index) = best_index {
            if let RTreeEntry::Node { mbr, child } = &mut node.entries[best_index] {
                *mbr = mbr.union(entry.mbr());
                insert_entry_node(child, entry);
                *mbr = compute_group_mbr(&child.entries);
            }
        } else {
            node.entries.push(entry);
        }
    }
}

/// Splits a vector of entries into two groups using a simple linear split.
fn split_entries<T: RTreeObject>(
    entries: Vec<RTreeEntry<T>>,
    _max_entries: usize,
) -> (Vec<RTreeEntry<T>>, Vec<RTreeEntry<T>>) {
    let mut entries = entries;
    if entries.len() < 2 {
        return (entries, Vec::new());
    }
    // For simplicity, choose the first two entries as seeds.
    let seed1 = entries.remove(0);
    let seed2 = entries.remove(0);
    let mut group1 = vec![seed1];
    let mut group2 = vec![seed2];

    for entry in entries {
        let mbr1 = compute_group_mbr(&group1);
        let mbr2 = compute_group_mbr(&group2);
        let enlargement1 = mbr1.enlargement(entry.mbr());
        let enlargement2 = mbr2.enlargement(entry.mbr());
        if enlargement1 < enlargement2 {
            group1.push(entry);
        } else {
            group2.push(entry);
        }
    }
    (group1, group2)
}

/// Computes the union (combined bounding volume) of a group of entries.
fn compute_group_mbr<T: RTreeObject>(entries: &Vec<RTreeEntry<T>>) -> T::B {
    let mut iter = entries.iter();
    let first = iter
        .next()
        .expect("At least one entry must be present")
        .mbr()
        .clone();
    iter.fold(first, |acc, entry| acc.union(entry.mbr()))
}

/// Recursively searches a node for all leaf entries whose bounding volume intersects the query.
/// Matching objects are added to `result`.
fn search_node<'a, T: RTreeObject>(node: &'a RTreeNode<T>, query: &T::B, result: &mut Vec<&'a T>) {
    if node.is_leaf {
        for entry in &node.entries {
            if let RTreeEntry::Leaf { mbr, object } = entry {
                if mbr.intersects(query) {
                    result.push(object);
                }
            }
        }
    } else {
        for entry in &node.entries {
            if let RTreeEntry::Node { mbr, child } = entry {
                if mbr.intersects(query) {
                    search_node(child, query, result);
                }
            }
        }
    }
}

// --- Implementations of RTreeObject for Point2D and Point3D ---

impl<T> RTreeObject for Point2D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    type B = Rectangle;
    fn mbr(&self) -> Self::B {
        // A 2D point is represented as a zero–area rectangle.
        Rectangle {
            x: self.x,
            y: self.y,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl<T> RTreeObject for Point3D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    type B = Cube;
    fn mbr(&self) -> Self::B {
        // A 3D point is represented as a zero–volume cube.
        Cube {
            x: self.x,
            y: self.y,
            z: self.z,
            width: 0.0,
            height: 0.0,
            depth: 0.0,
        }
    }
}
