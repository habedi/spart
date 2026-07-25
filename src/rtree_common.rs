use crate::geometry::BoundingVolume;
use std::cmp::Ordering;

/// Abstraction over an entry in a spatial tree (R-tree family).
pub trait EntryAccess {
    type BV: BoundingVolume + Clone;
    type Node: NodeAccess<Entry = Self>;
    type Obj;

    fn mbr(&self) -> &Self::BV;

    fn as_leaf_obj(&self) -> Option<&Self::Obj>;

    fn child(&self) -> Option<&Self::Node>;

    fn child_mut(&mut self) -> Option<&mut Self::Node>;

    fn set_mbr(&mut self, new_mbr: Self::BV);

    /// Consume the entry and return its child node if it is a Node entry.
    fn into_child(self) -> Option<Box<Self::Node>>
    where
        Self: Sized;
}

/// Abstraction over a node in a spatial tree (R-tree family).
pub trait NodeAccess {
    type Entry: EntryAccess;

    fn is_leaf(&self) -> bool;

    fn entries(&self) -> &Vec<Self::Entry>;

    fn entries_mut(&mut self) -> &mut Vec<Self::Entry>;
}

/// Height of `node` counted from the leaves: a leaf node has height 0, its parent height 1, ...
///
/// Heights are measured from the bottom on purpose. Levels numbered from the root shift every time
/// the tree grows a new root, which makes a level recorded before a split meaningless afterwards;
/// heights measured from the leaves stay valid.
pub fn node_height<N>(node: &N) -> usize
where
    N: NodeAccess,
    N::Entry: EntryAccess<Node = N>,
{
    let mut height = 0;
    let mut current = node;
    while let Some(child) = current.entries().first().and_then(EntryAccess::child) {
        height += 1;
        current = child;
    }
    height
}

/// Generic helper to compute the group MBR of a slice of entries.
pub fn compute_group_mbr<E: EntryAccess>(entries: &[E]) -> Option<E::BV> {
    let mut iter = entries.iter();
    let first = iter.next()?.mbr().clone();
    Some(iter.fold(first, |acc, entry| acc.union(entry.mbr())))
}

/// Generic range search on a node.
///
/// Dispatch is on the *entry* kind rather than on `NodeAccess::is_leaf`, so a search can never
/// silently skip part of the tree because a node's leaf flag disagrees with its contents.
pub fn search_node<'a, N>(
    node: &'a N,
    query: &<N::Entry as EntryAccess>::BV,
    result: &mut Vec<&'a <N::Entry as EntryAccess>::Obj>,
) where
    N: NodeAccess,
{
    for entry in node.entries() {
        if !entry.mbr().intersects(query) {
            continue;
        }
        if let Some(obj) = entry.as_leaf_obj() {
            result.push(obj);
        } else if let Some(child) = entry.child() {
            search_node(child, query, result);
        }
    }
}

/// Generic delete logic that mirrors both R-tree and R*-tree implementations.
///
/// Removes at most one object equal to `object` and returns whether it removed one.
///
/// `height` is the height of `node` counted from the leaves: a leaf node has height 0, its parent
/// height 1, and so on. When a child underflows it is detached and its entries are pushed onto
/// `reinsert_list` paired with **the height of the node they came from**. Callers must re-attach
/// each entry to a node at exactly that height: an entry moved to the wrong level either puts a
/// subtree where an object belongs or an object where a subtree belongs, and the tree stops being
/// uniformly deep.
pub fn delete_entry<N>(
    node: &mut N,
    object: &<N::Entry as EntryAccess>::Obj,
    object_mbr: &<N::Entry as EntryAccess>::BV,
    min_entries: usize,
    height: usize,
    reinsert_list: &mut Vec<(N::Entry, usize)>,
) -> bool
where
    N: NodeAccess,
    <N as NodeAccess>::Entry: EntryAccess,
    <<N as NodeAccess>::Entry as EntryAccess>::BV: Clone,
    <<N as NodeAccess>::Entry as EntryAccess>::Obj: PartialEq,
{
    if node.is_leaf() {
        let entries = node.entries_mut();
        let found = entries
            .iter()
            .position(|e| e.as_leaf_obj().is_some_and(|o| o == object));
        return match found {
            Some(pos) => {
                entries.remove(pos);
                true
            }
            None => false,
        };
    }

    let child_height = height.saturating_sub(1);
    let entries = node.entries_mut();
    let mut deleted = false;
    let mut underfull: Option<usize> = None;

    for (i, entry) in entries.iter_mut().enumerate() {
        if !entry.mbr().intersects(object_mbr) {
            continue;
        }
        let Some(child) = entry.child_mut() else {
            continue;
        };
        if !delete_entry(
            child,
            object,
            object_mbr,
            min_entries,
            child_height,
            reinsert_list,
        ) {
            continue;
        }
        deleted = true;
        if child.entries().len() < min_entries {
            underfull = Some(i);
        } else if let Some(new_mbr) = compute_group_mbr(child.entries()) {
            entry.set_mbr(new_mbr);
        }
        // Only one object is removed per call, so there is no reason to look at further siblings.
        // Scanning on would also delete one copy of a duplicated object from *every* subtree.
        break;
    }

    if let Some(index) = underfull {
        let removed = entries.remove(index);
        if let Some(child_box) = removed.into_child() {
            let mut child = *child_box;
            for entry in child.entries_mut().drain(..) {
                reinsert_list.push((entry, child_height));
            }
        }
    }

    deleted
}

/// Shared KNN candidate wrapper for priority queues.
#[derive(Debug)]
pub struct KnnCandidate<'a, E: EntryAccess> {
    pub dist: f64,
    pub entry: &'a E,
}

impl<E: EntryAccess> PartialEq for KnnCandidate<'_, E> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl<E: EntryAccess> Eq for KnnCandidate<'_, E> {}
impl<E: EntryAccess> Ord for KnnCandidate<'_, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .partial_cmp(&self.dist)
            .unwrap_or(Ordering::Equal)
    }
}
impl<E: EntryAccess> PartialOrd for KnnCandidate<'_, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Test-only check of the structural invariants every tree in the R-tree family must hold, and of
/// how many objects are actually reachable.
///
/// The invariants are what the search algorithms rely on:
///
/// * a leaf node holds only object entries and an interior node only subtree entries, because
///   mixing them used to make whole subtrees unreachable;
/// * every leaf sits at the same depth;
/// * no node holds more than `max_entries` entries, and (when `min_entries` is given) no node other
///   than the root holds fewer than that.
///
/// Returns the number of reachable objects so callers can assert nothing was lost.
#[cfg(test)]
pub(crate) fn assert_structure<N>(
    root: &N,
    max_entries: usize,
    min_entries: Option<usize>,
    context: &str,
) -> usize
where
    N: NodeAccess,
    N::Entry: EntryAccess<Node = N>,
{
    fn walk<N>(
        node: &N,
        depth: usize,
        is_root: bool,
        max_entries: usize,
        min_entries: Option<usize>,
        leaf_depths: &mut Vec<usize>,
        problems: &mut Vec<String>,
    ) -> usize
    where
        N: NodeAccess,
        N::Entry: EntryAccess<Node = N>,
    {
        let size = node.entries().len();
        if size > max_entries {
            problems.push(format!(
                "node at depth {depth} holds {size} entries, above max_entries={max_entries}"
            ));
        }
        if let Some(min) = min_entries {
            if !is_root && size < min {
                problems.push(format!(
                    "non-root node at depth {depth} holds {size} entries, below min_entries={min}"
                ));
            }
        }
        if node.is_leaf() {
            leaf_depths.push(depth);
        }

        let mut objects = 0;
        for entry in node.entries() {
            if entry.as_leaf_obj().is_some() {
                if !node.is_leaf() {
                    problems.push(format!(
                        "object entry inside interior node at depth {depth}"
                    ));
                }
                objects += 1;
            } else if let Some(child) = entry.child() {
                if node.is_leaf() {
                    problems.push(format!("subtree entry inside leaf node at depth {depth}"));
                }
                objects += walk(
                    child,
                    depth + 1,
                    false,
                    max_entries,
                    min_entries,
                    leaf_depths,
                    problems,
                );
            } else {
                problems.push(format!(
                    "entry at depth {depth} is neither an object nor a subtree"
                ));
            }
        }
        objects
    }

    let mut leaf_depths = Vec::new();
    let mut problems = Vec::new();
    let objects = walk(
        root,
        0,
        true,
        max_entries,
        min_entries,
        &mut leaf_depths,
        &mut problems,
    );
    leaf_depths.sort_unstable();
    leaf_depths.dedup();
    if leaf_depths.len() > 1 {
        problems.push(format!("leaves sit at differing depths: {leaf_depths:?}"));
    }
    assert!(problems.is_empty(), "{context}: {problems:#?}");
    objects
}

/// Writes the [`SpatialIndex`](crate::index::SpatialIndex) impl for one R-tree variant at one point
/// dimension.
///
/// The four impls needed (two variants, two dimensions) are pure delegation and differ only in the
/// names, so they are generated rather than copied. `contains` is the one method with a body: an
/// object is reachable only through its own bounding volume, so the lookup queries with that.
macro_rules! impl_rtree_spatial_index {
    ($tree:ident, $point:ident, $volume:ident) => {
        impl<T: std::fmt::Debug + Clone + PartialEq> $crate::index::SpatialIndex
            for $tree<$crate::geometry::$point<T>>
        {
            type Item = $crate::geometry::$point<T>;
            type Volume = $crate::geometry::$volume;

            fn len(&self) -> usize {
                $tree::len(self)
            }

            fn clear(&mut self) {
                $tree::clear(self);
            }

            fn contains(&self, item: &Self::Item) -> bool {
                use $crate::geometry::BoundedObject;
                $tree::range_search_bbox(self, &item.mbr())
                    .into_iter()
                    .any(|stored| stored == item)
            }

            /// Always `Ok(true)`; an R-tree has no boundary to fall outside of.
            fn insert(&mut self, item: Self::Item) -> Result<bool, $crate::errors::SpartError> {
                $tree::insert(self, item);
                Ok(true)
            }

            fn insert_bulk(
                &mut self,
                items: Vec<Self::Item>,
            ) -> Result<usize, $crate::errors::SpartError> {
                let count = items.len();
                $tree::insert_bulk(self, items);
                Ok(count)
            }

            fn delete(&mut self, item: &Self::Item) -> bool {
                $tree::delete(self, item)
            }

            fn knn_search<M: $crate::geometry::DistanceMetric<Self::Item>>(
                &self,
                query: &Self::Item,
                k: usize,
            ) -> Vec<&Self::Item> {
                <$tree<$crate::geometry::$point<T>>>::knn_search::<M>(self, query, k)
            }

            fn range_search<M: $crate::geometry::DistanceMetric<Self::Item>>(
                &self,
                query: &Self::Item,
                radius: f64,
            ) -> Vec<&Self::Item> {
                $tree::range_search::<M>(self, query, radius)
            }

            fn range_search_bbox(&self, query: &Self::Volume) -> Vec<&Self::Item> {
                $tree::range_search_bbox(self, query)
            }
        }
    };
}

pub(crate) use impl_rtree_spatial_index;

/// Writes the [`SpatialIndex`](crate::index::SpatialIndex) impl for one bounded tree.
///
/// The quadtree and octree impls are pure delegation and differ only in the names, so they are
/// generated rather than copied.
macro_rules! impl_bounded_spatial_index {
    ($tree:ident, $point:ident, $volume:ident) => {
        impl<T: Clone + PartialEq + std::fmt::Debug> $crate::index::SpatialIndex for $tree<T> {
            type Item = $crate::geometry::$point<T>;
            type Volume = $crate::geometry::$volume;

            fn len(&self) -> usize {
                $tree::len(self)
            }

            fn clear(&mut self) {
                $tree::clear(self);
            }

            fn contains(&self, item: &Self::Item) -> bool {
                $tree::contains(self, item)
            }

            /// `Ok(false)` for a point outside the tree's boundary; never an error.
            fn insert(&mut self, item: Self::Item) -> Result<bool, $crate::errors::SpartError> {
                Ok($tree::insert(self, item))
            }

            fn insert_bulk(
                &mut self,
                items: Vec<Self::Item>,
            ) -> Result<usize, $crate::errors::SpartError> {
                Ok($tree::insert_bulk(self, &items))
            }

            fn delete(&mut self, item: &Self::Item) -> bool {
                $tree::delete(self, item)
            }

            fn knn_search<M: $crate::geometry::DistanceMetric<Self::Item>>(
                &self,
                query: &Self::Item,
                k: usize,
            ) -> Vec<&Self::Item> {
                $tree::knn_search::<M>(self, query, k)
            }

            fn range_search<M: $crate::geometry::DistanceMetric<Self::Item>>(
                &self,
                query: &Self::Item,
                radius: f64,
            ) -> Vec<&Self::Item> {
                $tree::range_search::<M>(self, query, radius)
            }

            fn range_search_bbox(&self, query: &Self::Volume) -> Vec<&Self::Item> {
                $tree::range_search_bbox(self, query)
            }
        }
    };
}

pub(crate) use impl_bounded_spatial_index;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point2D, Rectangle};

    #[derive(Debug, Clone)]
    struct TestObj {
        id: i32,
        rect: Rectangle,
    }

    #[derive(Debug, Clone)]
    struct TestEntry {
        mbr: Rectangle,
        obj: Option<TestObj>,
        child: Option<Box<TestNode>>,
    }

    #[derive(Debug, Clone)]
    struct TestNode {
        entries: Vec<TestEntry>,
        is_leaf: bool,
    }

    impl EntryAccess for TestEntry {
        type BV = Rectangle;
        type Node = TestNode;
        type Obj = TestObj;

        fn mbr(&self) -> &Self::BV {
            &self.mbr
        }

        fn as_leaf_obj(&self) -> Option<&Self::Obj> {
            self.obj.as_ref()
        }

        fn child(&self) -> Option<&Self::Node> {
            self.child.as_deref()
        }

        fn child_mut(&mut self) -> Option<&mut Self::Node> {
            self.child.as_deref_mut()
        }

        fn set_mbr(&mut self, new_mbr: Self::BV) {
            self.mbr = new_mbr;
        }

        fn into_child(self) -> Option<Box<Self::Node>> {
            self.child
        }
    }

    impl NodeAccess for TestNode {
        type Entry = TestEntry;

        fn is_leaf(&self) -> bool {
            self.is_leaf
        }

        fn entries(&self) -> &Vec<Self::Entry> {
            &self.entries
        }

        fn entries_mut(&mut self) -> &mut Vec<Self::Entry> {
            &mut self.entries
        }
    }

    #[test]
    fn test_compute_group_mbr_contains_entries() {
        let a = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        };
        let b = Rectangle {
            x: 2.0,
            y: 2.0,
            width: 1.0,
            height: 1.0,
        };
        let entries = vec![
            TestEntry {
                mbr: a.clone(),
                obj: None,
                child: None,
            },
            TestEntry {
                mbr: b.clone(),
                obj: None,
                child: None,
            },
        ];
        let group_mbr = compute_group_mbr(&entries).expect("non-empty");
        let p1 = Point2D::new(0.0, 0.0, None::<()>);
        let p2 = Point2D::new(3.0, 3.0, None::<()>);
        assert!(group_mbr.contains(&p1));
        assert!(group_mbr.contains(&p2));
    }

    #[test]
    fn test_search_node_returns_intersecting_objects() {
        let obj_a = TestObj {
            id: 1,
            rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 2.0,
                height: 2.0,
            },
        };
        let obj_b = TestObj {
            id: 2,
            rect: Rectangle {
                x: 5.0,
                y: 5.0,
                width: 1.0,
                height: 1.0,
            },
        };
        let node = TestNode {
            is_leaf: true,
            entries: vec![
                TestEntry {
                    mbr: obj_a.rect.clone(),
                    obj: Some(obj_a.clone()),
                    child: None,
                },
                TestEntry {
                    mbr: obj_b.rect.clone(),
                    obj: Some(obj_b.clone()),
                    child: None,
                },
            ],
        };
        let query = Rectangle {
            x: -1.0,
            y: -1.0,
            width: 3.0,
            height: 3.0,
        };
        let mut result = Vec::new();
        search_node(&node, &query, &mut result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
    }
}
