//! ## Bounded Nearest-neighbor Heap
//!
//! Every tree in this crate runs the same k-nearest-neighbor accumulation: keep the `k` closest
//! items seen so far, and prune any subtree that cannot beat the farthest one kept. This module
//! holds that logic once. It used to be spelled out separately in each tree, including four
//! `HeapItem` structs declared inside function bodies with four trait impls apiece.

use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// One candidate held by a [`KnnHeap`].
struct Candidate<T> {
    /// Distance to the query, squared. Only compared, never returned, so the caller decides whether
    /// it is squared Euclidean or something else.
    distance: OrderedFloat<f64>,
    /// Insertion order, so items at equal distance come out in a deterministic order rather than
    /// one that depends on the heap's internal layout.
    seq: u64,
    item: T,
}

impl<T> Candidate<T> {
    fn key(&self) -> (OrderedFloat<f64>, u64) {
        (self.distance, self.seq)
    }
}

impl<T> PartialEq for Candidate<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<T> Eq for Candidate<T> {}

impl<T> Ord for Candidate<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

impl<T> PartialOrd for Candidate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Keeps the `k` nearest items offered to it.
///
/// The items are held in a max-heap ordered by distance, so the one evicted to make room is always
/// the farthest. [`KnnHeap::worst`] reports the distance a candidate has to beat, which lets search
/// code prune against a single value without separately handling a partly filled or zero-sized heap.
pub(crate) struct KnnHeap<T> {
    k: usize,
    seq: u64,
    heap: BinaryHeap<Candidate<T>>,
}

impl<T> KnnHeap<T> {
    /// Creates a heap that will keep at most `k` items.
    pub(crate) fn new(k: usize) -> Self {
        KnnHeap {
            k,
            seq: 0,
            heap: BinaryHeap::with_capacity(k.min(64)),
        }
    }

    /// The distance a candidate must beat to be kept.
    ///
    /// `f64::INFINITY` while the heap still has room, so callers prune nothing until they have `k`
    /// items. `f64::NEG_INFINITY` when `k` is zero, so callers prune everything: nothing can be
    /// nearer than negative infinity, and a search for zero neighbors must not walk the tree.
    pub(crate) fn worst(&self) -> f64 {
        if self.k == 0 {
            return f64::NEG_INFINITY;
        }
        if self.heap.len() < self.k {
            return f64::INFINITY;
        }
        self.heap
            .peek()
            .map_or(f64::INFINITY, |candidate| candidate.distance.into_inner())
    }

    /// Keeps `item` if it is nearer than the current worst, evicting that worst to make room.
    pub(crate) fn offer(&mut self, distance: f64, item: T) {
        if self.k == 0 {
            return;
        }
        if self.heap.len() == self.k {
            if distance >= self.worst() {
                return;
            }
            self.heap.pop();
        }
        self.seq += 1;
        self.heap.push(Candidate {
            distance: OrderedFloat(distance),
            seq: self.seq,
            item,
        });
    }

    /// The items kept, nearest first.
    pub(crate) fn into_sorted_vec(self) -> Vec<T> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|candidate| candidate.item)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keeps_the_nearest_k_in_order() {
        let mut heap = KnnHeap::new(3);
        for (d, name) in [(5.0, "e"), (1.0, "a"), (4.0, "d"), (2.0, "b"), (3.0, "c")] {
            heap.offer(d, name);
        }
        assert_eq!(heap.into_sorted_vec(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_worst_is_infinite_until_full() {
        let mut heap = KnnHeap::new(2);
        assert_eq!(heap.worst(), f64::INFINITY);
        heap.offer(1.0, "a");
        assert_eq!(heap.worst(), f64::INFINITY);
        heap.offer(4.0, "b");
        assert_eq!(heap.worst(), 4.0);
        // A farther candidate is refused, and a nearer one evicts the worst.
        heap.offer(9.0, "c");
        assert_eq!(heap.worst(), 4.0);
        heap.offer(2.0, "d");
        assert_eq!(heap.worst(), 2.0);
        assert_eq!(heap.into_sorted_vec(), vec!["a", "d"]);
    }

    /// `worst` has to prune rather than admit when no neighbors are wanted, or every tree walks
    /// itself in full to answer a query for zero neighbors.
    #[test]
    fn test_zero_k_keeps_nothing_and_prunes_everything() {
        let mut heap: KnnHeap<&str> = KnnHeap::new(0);
        heap.offer(1.0, "a");
        assert_eq!(heap.worst(), f64::NEG_INFINITY);
        assert!(
            0.0 > heap.worst(),
            "any candidate distance must fail the prune test"
        );
        assert!(heap.into_sorted_vec().is_empty());
    }

    /// Equal distances must come out in insertion order rather than heap order.
    #[test]
    fn test_ties_are_deterministic() {
        let mut heap = KnnHeap::new(4);
        for name in ["a", "b", "c", "d"] {
            heap.offer(7.0, name);
        }
        assert_eq!(heap.into_sorted_vec(), vec!["a", "b", "c", "d"]);
    }

    /// A tie at the boundary keeps whichever arrived first, so results stay stable.
    #[test]
    fn test_equal_distance_does_not_evict() {
        let mut heap = KnnHeap::new(1);
        heap.offer(3.0, "first");
        heap.offer(3.0, "second");
        assert_eq!(heap.into_sorted_vec(), vec!["first"]);
    }
}
