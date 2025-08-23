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

/// Generic helper to compute the group MBR of a slice of entries.
pub fn compute_group_mbr<E: EntryAccess>(entries: &[E]) -> Option<E::BV> {
    let mut iter = entries.iter();
    let first = iter.next()?.mbr().clone();
    Some(iter.fold(first, |acc, entry| acc.union(entry.mbr())))
}

/// Generic range search on a node.
pub fn search_node<'a, N>(
    node: &'a N,
    query: &<N::Entry as EntryAccess>::BV,
    result: &mut Vec<&'a <N::Entry as EntryAccess>::Obj>,
) where
    N: NodeAccess,
{
    if node.is_leaf() {
        for entry in node.entries() {
            if let Some(obj) = entry.as_leaf_obj() {
                if entry.mbr().intersects(query) {
                    result.push(obj);
                }
            }
        }
    } else {
        for entry in node.entries() {
            if let Some(child) = entry.child() {
                if entry.mbr().intersects(query) {
                    search_node(child, query, result);
                }
            }
        }
    }
}

/// Generic delete logic that mirrors both R-tree and R*-tree implementations.
pub fn delete_entry<N>(
    node: &mut N,
    object: &<N::Entry as EntryAccess>::Obj,
    object_mbr: &<N::Entry as EntryAccess>::BV,
    min_entries: usize,
    reinsert_list: &mut Vec<N::Entry>,
) -> bool
where
    N: NodeAccess,
    <N as NodeAccess>::Entry: EntryAccess,
    <<N as NodeAccess>::Entry as EntryAccess>::BV: Clone,
    <<N as NodeAccess>::Entry as EntryAccess>::Obj: PartialEq,
{
    let mut deleted = false;
    if node.is_leaf() {
        let entries = node.entries_mut();
        let initial_len = entries.len();
        entries.retain(|e| match e.as_leaf_obj() {
            Some(o) => o != object,
            None => true,
        });
        deleted = entries.len() < initial_len;
    } else {
        let entries = node.entries_mut();
        let mut to_delete_indices = Vec::new();
        for (i, entry) in entries.iter_mut().enumerate() {
            // Only descend into child nodes if MBR intersects object MBR
            let do_descend = {
                let mbr_clone = entry.mbr().clone();
                mbr_clone.intersects(object_mbr)
            };
            if do_descend {
                if let Some(child) = entry.child_mut() {
                    if delete_entry(child, object, object_mbr, min_entries, reinsert_list) {
                        deleted = true;
                        if child.entries().len() < min_entries {
                            to_delete_indices.push(i);
                        } else if let Some(new_mbr) = compute_group_mbr(child.entries()) {
                            entry.set_mbr(new_mbr);
                        }
                    }
                }
            }
        }

        // Remove underfilled children and reinsert their entries
        for &index in to_delete_indices.iter().rev() {
            // We need to move the entry out to get ownership and extract its child
            let removed = entries.remove(index);
            if let Some(child_box) = removed.into_child() {
                // Move all child entries into the reinsert list
                let mut child = *child_box;
                reinsert_list.append(child.entries_mut());
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
