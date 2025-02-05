use crate::geometry::{BoundingVolume, Cube, Point2D, Point3D, Rectangle};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::{debug, info};

pub trait RTreeObject: std::fmt::Debug {
    type B: BoundingVolume + std::fmt::Debug;
    fn mbr(&self) -> Self::B;
}

#[derive(Debug, Clone)]
pub enum RTreeEntry<T: RTreeObject> {
    Leaf { mbr: T::B, object: T },
    Node { mbr: T::B, child: Box<RTreeNode<T>> },
}

impl<T: RTreeObject> RTreeEntry<T> {
    pub fn mbr(&self) -> &T::B {
        match self {
            RTreeEntry::Leaf { mbr, .. } => mbr,
            RTreeEntry::Node { mbr, .. } => mbr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RTreeNode<T: RTreeObject> {
    pub entries: Vec<RTreeEntry<T>>,
    pub is_leaf: bool,
}

#[derive(Debug)]
pub struct RTree<T: RTreeObject> {
    root: RTreeNode<T>,
    max_entries: usize,
}

impl<T: RTreeObject> RTree<T> {
    pub fn new(max_entries: usize) -> Self {
        info!("Creating new RTree with max_entries: {}", max_entries);
        RTree {
            root: RTreeNode {
                entries: Vec::new(),
                is_leaf: true,
            },
            max_entries,
        }
    }

    pub fn insert(&mut self, object: T) {
        info!("Inserting object into RTree: {:?}", object);
        let entry = RTreeEntry::Leaf {
            mbr: object.mbr(),
            object,
        };
        insert_entry_node(&mut self.root, entry);
        if self.root.entries.len() > self.max_entries {
            info!("Root has exceeded max_entries; splitting root");
            self.split_root();
        }
    }

    fn split_root(&mut self) {
        info!("Splitting root node");
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

    pub fn range_search(&self, query: &T::B) -> Vec<&T> {
        info!("Performing range search with query: {:?}", query);
        let mut result = Vec::new();
        search_node(&self.root, query, &mut result);
        result
    }
}

fn insert_entry_node<T: RTreeObject>(node: &mut RTreeNode<T>, entry: RTreeEntry<T>) {
    if node.is_leaf {
        debug!("Inserting entry into leaf node");
        node.entries.push(entry);
    } else {
        let mut best_index: Option<usize> = None;
        let mut best_enlargement = f64::INFINITY;
        for (i, child_entry) in node.entries.iter().enumerate() {
            if let RTreeEntry::Node { mbr, .. } = child_entry {
                let enlargement = mbr.enlargement(entry.mbr());
                if enlargement < best_enlargement {
                    best_enlargement = enlargement;
                    best_index = Some(i);
                } else if (enlargement - best_enlargement).abs() < std::f64::EPSILON {
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

fn split_entries<T: RTreeObject>(
    entries: Vec<RTreeEntry<T>>,
    _max_entries: usize,
) -> (Vec<RTreeEntry<T>>, Vec<RTreeEntry<T>>) {
    let mut entries = entries;
    if entries.len() < 2 {
        return (entries, Vec::new());
    }
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

fn compute_group_mbr<T: RTreeObject>(entries: &Vec<RTreeEntry<T>>) -> T::B {
    let mut iter = entries.iter();
    let first = iter
        .next()
        .expect("At least one entry must be present")
        .mbr()
        .clone();
    iter.fold(first, |acc, entry| acc.union(entry.mbr()))
}

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

impl<T: RTreeObject> RTree<T>
where
    T: PartialEq,
{
    pub fn delete(&mut self, object: &T) -> bool {
        info!("Attempting to delete object: {:?}", object);
        let found = delete_entry(&mut self.root, object);
        if !self.root.is_leaf && self.root.entries.len() == 1 {
            if let RTreeEntry::Node { child, .. } = self.root.entries.pop().unwrap() {
                self.root = *child;
            }
        }
        found
    }
}

fn delete_entry<T: RTreeObject>(node: &mut RTreeNode<T>, object: &T) -> bool
where
    T: PartialEq,
{
    if node.is_leaf {
        let initial_len = node.entries.len();
        node.entries.retain(|entry| {
            if let RTreeEntry::Leaf {
                object: ref obj, ..
            } = entry
            {
                obj != object
            } else {
                true
            }
        });
        initial_len != node.entries.len()
    } else {
        let mut found = false;
        for entry in &mut node.entries {
            if let RTreeEntry::Node { child, .. } = entry {
                if delete_entry(child, object) {
                    if !child.entries.is_empty() {
                        let new_mbr = compute_group_mbr(&child.entries);
                        if let RTreeEntry::Node { ref mut mbr, .. } = entry {
                            *mbr = new_mbr;
                        }
                    }
                    found = true;
                }
            }
        }
        node.entries.retain(|entry| match entry {
            RTreeEntry::Node { child, .. } => !child.entries.is_empty(),
            _ => true,
        });
        found
    }
}

impl<T: std::fmt::Debug> RTreeObject for Point2D<T> {
    type B = Rectangle;
    fn mbr(&self) -> Self::B {
        Rectangle {
            x: self.x,
            y: self.y,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl<T: std::fmt::Debug> RTreeObject for Point3D<T> {
    type B = Cube;
    fn mbr(&self) -> Self::B {
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

impl Rectangle {
    pub fn min_distance<T>(&self, point: &Point2D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug)]
struct Candidate2D<'a, T: std::fmt::Debug> {
    dist: f64,
    entry: CandidateEntry2D<'a, T>,
}

#[derive(Debug)]
enum CandidateEntry2D<'a, T: std::fmt::Debug> {
    Node(&'a RTreeNode<Point2D<T>>),
    Leaf(&'a Point2D<T>),
}

impl<'a, T: std::fmt::Debug> PartialEq for Candidate2D<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl<'a, T: std::fmt::Debug> Eq for Candidate2D<'a, T> {}
impl<'a, T: std::fmt::Debug> PartialOrd for Candidate2D<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}
impl<'a, T: std::fmt::Debug> Ord for Candidate2D<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: std::fmt::Debug> RTree<Point2D<T>> {
    pub fn knn_search(&self, query: &Point2D<T>, k: usize) -> Vec<&Point2D<T>> {
        let mut heap = BinaryHeap::new();
        heap.push(Candidate2D {
            dist: compute_group_mbr(&self.root.entries).min_distance(query),
            entry: CandidateEntry2D::Node(&self.root),
        });
        let mut result = Vec::new();
        while let Some(Candidate2D { entry, .. }) = heap.pop() {
            match entry {
                CandidateEntry2D::Leaf(obj) => {
                    result.push(obj);
                    if result.len() == k {
                        break;
                    }
                }
                CandidateEntry2D::Node(node) => {
                    for entry in &node.entries {
                        match entry {
                            RTreeEntry::Leaf { mbr, object } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate2D {
                                    dist: d,
                                    entry: CandidateEntry2D::Leaf(object),
                                });
                            }
                            RTreeEntry::Node { mbr, child } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate2D {
                                    dist: d,
                                    entry: CandidateEntry2D::Node(child),
                                });
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

impl Cube {
    pub fn min_distance<T>(&self, point: &Point3D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        let dz = if point.z < self.z {
            self.z - point.z
        } else if point.z > self.z + self.depth {
            point.z - (self.z + self.depth)
        } else {
            0.0
        };
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
struct Candidate3D<'a, T: std::fmt::Debug> {
    dist: f64,
    entry: CandidateEntry3D<'a, T>,
}

#[derive(Debug)]
enum CandidateEntry3D<'a, T: std::fmt::Debug> {
    Node(&'a RTreeNode<Point3D<T>>),
    Leaf(&'a Point3D<T>),
}

impl<'a, T: std::fmt::Debug> PartialEq for Candidate3D<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl<'a, T: std::fmt::Debug> Eq for Candidate3D<'a, T> {}
impl<'a, T: std::fmt::Debug> PartialOrd for Candidate3D<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}
impl<'a, T: std::fmt::Debug> Ord for Candidate3D<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: std::fmt::Debug> RTree<Point3D<T>> {
    pub fn knn_search(&self, query: &Point3D<T>, k: usize) -> Vec<&Point3D<T>> {
        let mut heap = BinaryHeap::new();
        let root_mbr = compute_group_mbr(&self.root.entries);
        heap.push(Candidate3D {
            dist: root_mbr.min_distance(query),
            entry: CandidateEntry3D::Node(&self.root),
        });
        let mut result = Vec::new();
        while let Some(Candidate3D { entry, .. }) = heap.pop() {
            match entry {
                CandidateEntry3D::Leaf(obj) => {
                    result.push(obj);
                    if result.len() == k {
                        break;
                    }
                }
                CandidateEntry3D::Node(node) => {
                    for entry in &node.entries {
                        match entry {
                            RTreeEntry::Leaf { mbr, object } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate3D {
                                    dist: d,
                                    entry: CandidateEntry3D::Leaf(object),
                                });
                            }
                            RTreeEntry::Node { mbr, child } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate3D {
                                    dist: d,
                                    entry: CandidateEntry3D::Node(child),
                                });
                            }
                        }
                    }
                }
            }
        }
        result
    }
}
