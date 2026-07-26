# Changelog

All notable changes to Spart are recorded here.

## 0.6.0

A correctness and API release. Every tree family had at least one defect that lost or corrupted query results, and the public API has been reshaped
around a single trait. **This release is not source-compatible with 0.5.2, and it cannot read trees serialized by 0.5.2.**

### Fixed

- R*-tree: forced reinsertion put the entries it pulled out back at the wrong level, burying whole subtrees inside leaf nodes. A range search over
  1000 inserted points could see one of them. Reinserted entries now carry the height they came from, counted from the leaves so the height stays
  valid when the tree grows a new root.
- R-tree: only the root ever split, so the tree stayed two levels deep no matter how many objects it held and every query scanned about half the data.
  Splits now propagate up from the node that overflowed, and the split honors `min_entries`.
- R-tree and R*-tree: `insert_bulk` appended entries of the wrong kind to the root, which made either the new batch or the objects already stored
  unreachable. Bulk loading now packs a uniformly deep tree into an empty tree and inserts individually into a populated one.
- Quadtree and Octree: more coincident points than a node's capacity subdivided until floating-point rounding left a point matching a parent but no
  child, which reached `unreachable!()` on `insert`
  and silently dropped every point on `insert_bulk`. Child routing now compares against node midpoints, children tile their parent exactly, and
  subdivision stops at a depth cap.
- Kd-tree: incremental inserts were never rebalanced, so sorted input built a tree of depth *n* and every recursive walk over it, including dropping
  it, overflowed the stack past roughly 50k points. Subtree sizes are now tracked and an unbalanced subtree is rebuilt.
- Kd-tree: `range_search` squared the radius without checking its sign, so a negative radius behaved like a positive one. It now returns nothing, as
  the other trees already did.
- Kd-tree: the dimension was silently cleared when the tree became empty, letting the next insert establish a different one.
- Kd-tree: `insert_bulk` set the tree's dimension before validating the batch.
- `delete` removed one copy of a duplicated object from *every* subtree it appeared in rather than one copy in total.
- `Rectangle::union` and `Cube::union` inflated the result by an epsilon, so `enlargement` was never zero for an already-contained volume and
  recomputed bounding volumes crept outwards. The extent is now exact to the last representable bit.
- A point's bounding volume is now exactly zero-sized, so `range_search_bbox` means containment on every tree. It previously reported points up to
  1e-10 outside the query on the R-tree family.
- `k = 0` no longer walks the tree before returning nothing.
- Quadtree and Octree nearest-neighbor search now visits children nearest first. In a fixed spatial order the pruning bound was still loose when the
  far children were tested, so most of them were entered anyway: over 5000 uniformly scattered points a single-neighbor query computed 315 distances
  where it now computes 12.
- The `setup_tracing` feature did not compile at all.

### Changed

- **Breaking:** `RTreeObject` and `RStarTreeObject` are replaced by a single `geometry::BoundedObject`, whose associated type is named `Volume` rather
  than `B`.
- **Breaking:** query methods return references. `Quadtree`, `Octree`, and `KdTree` searches changed from `Vec<Point>` to `Vec<&Point>`; clone what
  you keep.
- **Breaking:** `geometry::HeapItem` is removed. It was an internal accumulator that had no reason to be public.
- **Breaking:** `RTreeNode`, `RTreeEntry`, `RStarTreeNode`, and `RStarTreeEntry` are now crate-private.
- **Breaking:** `Quadtree::insert_bulk` and `Octree::insert_bulk` return the number of points stored instead of `()`, since they skip points outside
  the boundary.
- **Breaking:** the serialized layout changed. Trees written by 0.5.2 cannot be loaded by 0.6.0.
- **Breaking:** the `setup_tracing` feature no longer installs a subscriber before `main`. Call `spart::init_tracing()` instead, which honours `DEBUG_SPART` exactly as before and returns whether it installed anything. A library taking the global subscriber slot without being asked is not its decision to make, and doing it before `main` required running unsafe code before the runtime was up. The `ctor` dependency is gone with it.
- `SpartError` now derives `Clone`, `PartialEq`, and `Eq`.
- `#![deny(missing_docs)]` and `#![forbid(unsafe_code)]` are enabled. The crate contains no `unsafe`.
- The crate is verified to build for `wasm32-unknown-unknown` and `wasm32-wasip1`; `make wasm` checks
  both. Dev-dependencies do not compile for those targets, so the check covers the library only.

### Added

- `index::SpatialIndex`, implemented by all five trees, giving one interface over `len`, `is_empty`, `clear`, `contains`, `insert`, `insert_bulk`,
  `delete`, `knn_search`, `range_search`, and `range_search_bbox`.
- `len`, `is_empty`, `clear`, and `contains` on every tree, in Rust and in the Python bindings.
- `range_search_bbox` on the Kd-tree, and on every Python binding.
- Completeness tests comparing every query against brute force, structural invariant checks run after each insert and delete, and one generic harness
  holding all five trees to the same contract.
