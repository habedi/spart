# AGENTS.md

This file provides guidance to coding agents collaborating on this repository.

## Mission

Spart is a collection of space partitioning tree data structures for Rust, with Python bindings.
Priorities, in order:

1. Correct query behavior: a query must return every point it covers, not merely correct ones, and every tree must hold its structural invariants
   after each operation.
2. Clear boundaries between the shared geometric primitives, the individual tree implementations, and the Python bindings.
3. Reproducible, benchmark-backed performance; no premature optimization before correctness is covered.
4. Idiomatic Rust: ownership, zero-cost abstractions, and `unsafe` only where necessary and documented.

## Core Rules

- Use English for code, comments, docs, and tests.
- Prefer small, focused changes over broad rewrites.
- Keep the module boundaries: `geometry` owns the primitives and the traits over them, each tree module owns one data structure, `rtree_common` owns
  the logic shared by the two R-tree variants, and `pyspart` wraps the public API. Do not import across those boundaries in the wrong direction; see
  Dependency Boundaries.
- Keep all mutable state inside the tree values themselves; do not introduce module-level `static mut`, `lazy_static`, or `OnceLock` globals for
  runtime state. The only process-global state is the optional tracing subscriber behind the `setup_tracing` feature.
- A tree operation that cannot store a point must report it. Never drop a point silently; return `false`, return a count, or return an `Err`.
- Do not reach for `unwrap` or `expect` in library code. `make lint` denies both. Prefer an early return, a defensive fallback that keeps data
  reachable, or a documented `unreachable!` guarded by a check on the line above.
- Respect the MSRV in `rust-toolchain.toml` and `Cargo.toml` (1.85.0). Do not use standard-library APIs stabilized later; `geometry::span` manipulates
  bits by hand rather than calling `f64::next_up` for exactly this reason.
- Add comments only when they clarify a non-obvious tree invariant, a floating-point subtlety, or why a simpler formulation is wrong.
- Maintain the permissive license boundary of the crate (MIT or Apache-2.0). Do not add dependencies or statically link libraries with copyleft, weak
  copyleft, or source-available licenses (such as GPL, MPL, or SSPL).
- Format with `rustfmt` (`make format`) and lint with Clippy (`make lint`) before declaring a change done.

Quick examples:

- Good: add a `Quadtree::range_search_bbox` method next to the existing search methods, with a unit test and a brute-force comparison in
  `tests/test_completeness.rs`.
- Good: extend `geometry::BoundingVolume` with a default method and override it for `Rectangle` and `Cube`.
- Bad: duplicate the minimum-distance computation in a tree module instead of calling `HasMinDistance`.
- Bad: change a tree so a query returns a subset of the covered points, even if every returned point is correct.
- Bad: add a cargo dependency that pulls in a copyleft or source-available library.

## Writing Style

- Use Oxford commas in inline lists: "a, b, and c" not "a, b, c".
- Do not use em dashes. Restructure the sentence, or use a colon or semicolon instead.
- Avoid colorful adjectives and adverbs. Write "range query" not "blazing range query".
- Prefer noun phrases for checklist items over imperative verbs. Write "brute-force comparison" not "compare against brute force".
- Headings in Markdown files must be in title case: "Build from Source" not "Build from source". Minor words (a, an, the, and, but, or, for, in, on,
  at, to, by, of) stay lowercase unless they are the first word.
- Do not bold the lead-in of a list item. Write "Uniform depth: ..." not "**Uniform depth**: ...".
- Use sentence case for the lead-in of a list item. Write "Node splitting: ..." not "Node Splitting: ...". Proper nouns keep their capitals.
- Capitalize only the first part of a hyphenated compound: "Nearest-neighbor Search" in a heading, "Breadth-first" at the start of a sentence, and
  "breadth-first search" elsewhere. Never write "Breadth-First".
- Start each sentence with a capital letter, capitalize proper nouns (Rust, Python, Clippy, Criterion, PyO3), and leave common nouns lowercase in the
  middle of a sentence.
- Write correct and complete sentences.
- Avoid made-up words, abbreviations, and colons in the middle of sentences.
- Use participial phrases scarcely.

## Repository Layout

Do not invent modules that do not yet exist, but do place new modules according to this map.

- `src/lib.rs`: module declarations only. `errors`, `geometry`, `kdtree`, `octree`, `quadtree`, `rstar_tree`, and `rtree` are public; `logging` and
  `rtree_common` are private.
- `src/geometry.rs`: shared primitives and the traits over them. `Point2D`, `Point3D`, `Rectangle`, and `Cube`, plus `DistanceMetric`,
  `EuclideanDistance`, `BSPBounds`, `BoundingVolume`, `HasMinDistance`, `BoundingVolumeFromPoint`, and `HeapItem`. Also the `pub(crate)` `span` helper
  and the module-private `axis_distance`.
- `src/errors.rs`: `SpartError`, the single error type. Every fallible constructor and insert returns it.
- `src/logging.rs`: optional tracing subscriber installed before `main` by a `ctor`, behind the `setup_tracing` feature and driven by the
  `DEBUG_SPART` environment variable.
- `src/quadtree.rs`: `Quadtree`, a 2D point quadtree over a `Rectangle` boundary with a per-node `capacity`.
- `src/octree.rs`: `Octree`, the 3D counterpart over a `Cube` boundary. Mirrors `quadtree.rs` structurally; a fix in one almost always belongs in the
  other.
- `src/kdtree.rs`: `KdTree` and the `KdPoint` trait. Balance is maintained by partial rebuilding, so nodes carry subtree sizes.
- `src/rtree.rs`: `RTree`, `RTreeNode`, `RTreeEntry`, and the `RTreeObject` trait. Guttman quadratic split.
- `src/rstar_tree.rs`: `RStarTree` and its node, entry, and object types. R*-tree choose-subtree, forced reinsertion, and margin-based split.
- `src/rtree_common.rs`: the `EntryAccess` and `NodeAccess` abstractions plus the logic shared by both R-tree variants (`node_height`,
  `compute_group_mbr`, `search_node`, `delete_entry`, and `KnnCandidate`). Private to the crate. Also holds the test-only `assert_structure` invariant
  checker.
- `tests/common.rs`: shared fixtures for the integration tests, included with `#[path]` by each of them.
- `tests/test_integration_*.rs`: one per tree, exercising insert, search, and delete through the public API.
- `tests/test_proptest_*.rs`: `proptest` properties per tree, plus `test_proptest_geometry.rs` for the primitives.
- `tests/test_completeness.rs`: brute-force comparisons. Every query must return exactly the covered points, checked against the set of points known
  to be live.
- `tests/test_serialization.rs`: serde round-trips for every tree, gated on the `serde` feature.
- `benches/main.rs`: the single Criterion bench target. Declares `mod shared` once and pulls in the `bench_*` modules.
- `benches/bench_*.rs`: modules of that target, one per operation. They are not bench targets of their own; `autobenches = false` in `Cargo.toml`
  enforces this.
- `benches/shared.rs`: bench data generators.
- `examples/`: one runnable example per tree, wired up as named `[[example]]` targets in `Cargo.toml`.
- `pyspart/`: PyO3 bindings, a separate crate with its own `Cargo.toml` that depends on the parent by path.
    - `pyspart/src/lib.rs`: module registration.
    - `pyspart/src/point2d.rs` and `point3d.rs`: the `Point2D` and `Point3D` classes. `pyspart/src/geometry.rs`: the `PyRectangle` and `PyCube`
      newtypes. `pyspart/src/types.rs`: the `PyData` payload type.
    - `pyspart/src/quadtree.rs`, `octree.rs`, `kdtree.rs`, `rtree.rs`, and `rstar_tree.rs`: one file per tree. The bounded trees expose `Quadtree` and
      `Octree`; the unbounded ones expose a class per dimension (`KdTree2D`, `KdTree3D`, `RTree2D`, `RTree3D`, `RStarTree2D`, and `RStarTree3D`).
    - `pyspart/pyspart.pyi`: type stubs. Keep them in step with the Rust signatures.
    - `pyspart/tests/`: pytest suites.
    - `pyspart/examples/`: runnable Python examples.
- `Cargo.toml`: crate manifest. All version pins live here.
- `Makefile`: developer workflow entry points.

## Testing Layout Rules

- Unit tests belong in `#[cfg(test)]` blocks inside the relevant source file. A test that needs to see private fields, such as a structural invariant
  check, has to live there.
- Tests that drive behavior only through the public API belong in `tests/`.
- Property-based tests (via `proptest`) belong in the matching `tests/test_proptest_*.rs`.
- A new completeness or brute-force comparison belongs in `tests/test_completeness.rs`, not in a per-tree integration file.
- Keep tests deterministic. `tests/test_completeness.rs` carries a small seeded generator for this reason; do not introduce an unseeded random source.
- If you move code across modules, move or rewrite the unit tests with it.
- Benchmark targets live in `benches/`; do not add `#[bench]` to source files, and do not add a new file there without wiring it into
  `benches/main.rs`.

## Architecture Constraints

### R-tree Family

These invariants are what the search algorithms rely on. `rtree_common::assert_structure` checks all of them and is called after every operation in
the unit tests; extend it rather than writing a new walker.

- Uniform depth: every leaf sits at the same depth.
- Entry kinds: a leaf node holds only object entries and an interior node only subtree entries. Mixing them is the failure mode to watch for, because
  it makes a whole subtree unreachable to a search that dispatches on the node's leaf flag.
- Node occupancy: no node holds more than `max_entries` entries, and no node other than the root holds fewer than `min_entries`.
- Heights are counted from the leaves: a leaf node has height 0, its parent height 1. Never number levels from the root. A level recorded before the
  tree grows a new root is meaningless afterwards, whereas a height measured from the leaves stays valid.
- `delete_entry` detaches an underfull child and returns its entries paired with the height they came from. Re-attach each one at exactly that height.
  Anything else buries a subtree inside a leaf or an object inside an interior node.
- `delete_entry` removes at most one object per call and stops at the first match. Scanning on would delete one copy of a duplicated object from every
  subtree.
- Splits propagate: an overfull node splits and hands a sibling entry to its parent, which adds it to its own entries. Only the root caller turns a
  sibling into a new level. A split that stops before reaching the root leaves the tree with unbounded leaves.
- Both groups of a split get at least `min_entries`, so the usual call with `max_entries + 1` entries cannot leave either group above `max_entries`.
- `search_node` dispatches on the entry kind, not on `NodeAccess::is_leaf`, so a search can never skip part of the tree because a node's flag
  disagrees with its contents.
- Forced reinsertion in the R*-tree happens at most once per height while a single object is being inserted, and overflow at the root always splits.
  Without both rules the insertion does not terminate.
- `insert_bulk` packs bottom-up only into an empty tree, tracking leaf-ness per level. Into a populated tree it inserts objects one at a time, because
  appending pre-built nodes to an existing tree cannot preserve a uniform depth.

### Quadtree and Octree

- Child routing compares a point against the node's midpoints and midplanes, never against each child's boundary. Boundary testing lets floating-point
  rounding produce a point that matches the parent and no child at all.
- Child boundaries are derived from the parent's own edges through `geometry::span`, so children tile the parent exactly. A gap of even one ULP lets a
  point be stored in a child whose boundary does not contain it, and search pruning then skips over it.
- Subdivision stops at `MAX_DEPTH` (32). Past that a node keeps its points and grows beyond `capacity`. Coincident points always land in the same
  child, so no further split can separate them and subdividing would recurse without end.
- `try_merge` considers only the node it is called on. `delete` calls it at every level on the way back up, so the path that changed is merged
  bottom-up already; recursing over the whole subtree makes each delete cost time proportional to the size of the tree.
- `delete` descends into the single routed child, since insertion put the point there and nowhere else.
- The child accessors return fixed-size arrays of `Option`, not a `Vec`. Query paths visit these per node, so an allocation there is a per-node cost.

### Kd-tree

- Nodes carry a subtree `size`, maintained by every insert and delete. Any code path that changes a child must call `update_size`.
- Balance is kept by partial rebuilding: after the search path changes, the highest subtree whose heavier child exceeds two thirds of it is rebuilt
  balanced. This is what bounds the height at a logarithm of the point count. Without it, sorted input builds a path and every recursive walk over the
  tree, including dropping it, overflows the stack.
- The split rule is strict on the left: a point goes left when its coordinate is less than the node's, and right otherwise. A search that finds
  equality on the split axis must look in both subtrees.
- The dimension is fixed by the first insert or by `with_dimension` and is never cleared, not even when the tree becomes empty. Clearing it would let
  the next insert silently establish a different dimension.

### Geometry

- `span(lo, hi)` returns the smallest extent for which `lo + extent >= hi`. Bounding volumes are stored as an origin plus an extent and containment
  compares against `origin + extent`, so a plain subtraction can round down and leave a volume that fails to contain the corner it was built from.
- `union` must stay tight, and therefore idempotent, so that `enlargement` is exactly zero for an already-contained volume. Do not widen a volume by a
  fudge factor to paper over a rounding problem; fix the extent instead.
- Minimum-distance logic lives in `HasMinDistance` and nowhere else. `min_distance_sq` exists so pruning code does not take a square root only to
  square it again; override it wherever you implement the trait.
- Object bounding volumes for points currently use a `1e-10` extent rather than zero, which lets a bounding-box query report a point up to `1e-10`
  outside the query. This is a known wart, not a design goal.

### Features and Compatibility

- Features are `serde`, `enable_log`, and `setup_tracing`; the default set is empty. Every combination has to compile and pass, so check
  `--all-features` and not just the default build.
- Adding or reordering a field of a serialized type breaks compatibility with data written by an earlier version, because bincode is positional. Such
  a change needs a version bump and a note in the release notes.
- Logging goes through `tracing` at `debug` and `info` level. Do not print to stdout or stderr from library code.
- Async is not used anywhere in the crate. Do not introduce a runtime or an `.await`.

## Dependency Boundaries

Target dependency direction:

1. `geometry` and `errors` sit at the bottom. They must not reference any tree module.
2. `rtree_common` may depend on `geometry`, but not on `rtree`, `rstar_tree`, or any other tree module. It reaches the two R-tree variants only
   through the `EntryAccess` and `NodeAccess` traits they implement.
3. Each tree module may depend on `geometry`, `errors`, and, for the two R-tree variants, `rtree_common`. No tree module may depend on another.
4. `pyspart` depends only on the public API of `spart`. It must not reimplement traversal, splitting, or search logic.

Lower-level modules must not know about higher-level modules. `quadtree` and `octree` are peers: keep them in step by editing both, not by having one
call the other.

## Component APIs

Each tree carries a payload type parameter and exposes the same shape of operations. Keep new methods consistent with these names across all five
trees.

### `Quadtree<T>` and `Octree<T>`

Bounded trees: a node subdivides once it exceeds `capacity`, and a point outside the root boundary is rejected.

- `new(boundary, capacity) -> Result<Self, SpartError>`: rejects a zero `capacity`.
- `insert(point) -> bool`: `false` when the point lies outside the boundary.
- `insert_bulk(points) -> usize`: the number of points stored; the rest lay outside the boundary.
- `knn_search::<M>(target, k)`, `range_search::<M>(center, radius)`, and `range_search_bbox(query)`.
- `delete(point) -> bool`.

### `KdTree<P: KdPoint>`

Unbounded and dimension-agnostic; the dimension comes from the points.

- `new()`, `with_dimension(k)`, and `contains(point)`.
- `insert(point) -> Result<(), SpartError>` and `insert_bulk(points) -> Result<(), SpartError>`, both returning `DimensionMismatch` on a wrong-sized
  point. Validate a whole batch before touching any state, so a rejected batch leaves the tree as it was.
- `knn_search::<M>(target, k)` and `range_search::<M>(center, radius)`, both returning owned points.
- `delete(point) -> bool`.

### `RTree<T: RTreeObject>` and `RStarTree<T: RStarTreeObject>`

Unbounded, keyed on the bounding volume an object reports through `mbr`.

- `new(max_entries) -> Result<Self, SpartError>`: rejects `max_entries` below 2. `min_entries` is derived as 40 percent of it.
- `insert(object)` and `insert_bulk(objects)`.
- `range_search_bbox(query)` and `range_search::<M>(query, radius)`, both returning references.
- `knn_search::<M>(query, k)`, implemented per point dimension.
- `delete(object) -> bool`, requiring `T: PartialEq`.
- `RStarTree::height()` reports the number of levels; a tree whose root is a leaf has height 1.

### Encapsulation Rule

`rtree_common` and `logging` are private modules and are not reachable from outside the crate. The `root`, `max_entries`, and `min_entries` fields of
the trees are private and must stay that way. Do not add a "just for now" accessor; add a test-only helper inside the module if a test needs internal
access.

`RTreeNode`, `RTreeEntry`, and their R*-tree counterparts are public with public fields, but no public signature mentions them: flipping all four
to `pub(crate)` leaves the crate, its tests, and the bindings compiling. Treat them as an unintended leak rather than a supported surface, and do
not build on them.

## Workflow

Before coding:

1. Identification of whether this is a geometry, single-tree, R-tree-family, bindings, or docs change.
2. Reading of the touched module and its tests, plus the peer module when the change touches `quadtree` or `octree`.

Implementation using red-green TDD:

1. A failing `#[test]` that describes the expected behavior (red). For an invariant, prefer a `proptest` property or a brute-force comparison in
   `tests/test_completeness.rs`.
2. Verification that the test fails for the right reason, running `cargo test -- <test_name>` (red).
3. The smallest implementation that makes the test pass (green).
4. Refactor while keeping tests green.
5. Narrowest relevant test while iterating, then the full checks below before declaring done.
6. `make format` before every commit.
7. Update of `README.md`, the rustdoc, or `pyspart/pyspart.pyi` if behavior or workflow changed.

Full checks before declaring a change done:

- `make test` runs the default feature set only. Also run `cargo test --all-features`, or a serde or tracing regression will go unnoticed.
- `make lint` lints the default target and feature set. Also run `cargo clippy --all-targets --all-features`, which covers the tests, benches, and
  examples.
- `cargo doc --all-features --no-deps` for a documentation change or a new doc link.

Additional validation when relevant:

- `make bench` for a performance-sensitive change. `make test-py` for anything reaching the bindings. `make run-examples` and `make run-py-examples`
  after a public API change.
- `make coverage` and `make nextest` are available for coverage and for a process-per-test run.
- `make audit` after touching dependencies.

## Testing Expectations

- No behavior change is complete without tests.
- A tree change needs coverage in both directions: soundness, meaning every returned point really is covered by the query, and completeness, meaning
  every covered point is returned. Completeness is the direction that is easy to forget and the direction where data loss hides.
- Structural invariants belong in a unit test that walks the tree after every operation, not in a single check at the end of a workload.
- Cover the smallest legal capacities (`max_entries` of 2, `capacity` of 1) and coincident points. These are where splitting and subdivision go wrong.
- Prefer targeted assertions (one field, one count, one round-trip) over broad snapshot tests.
- When uncertain about correctness, add or refine tests first.

## Documentation Expectations

- Public API docs are generated by `rustdoc` from the module docs and item docs in `src/`. Each of the five tree modules carries a runnable example in
  its module-level docs, and `geometry` carries one per primitive operation; keep a new tree or operation consistent with that.
- Almost every public item carries a doc comment. Document any item you add.
- `missing_docs` is not enabled, and a handful of items predate the convention: the `RTreeEntry` and `RStarTreeEntry` variants and their fields, and
  the crate root itself. Prefer filling one of these in over adding another.
- A doctest is a test. A change to a signature shown in a module example has to update that example.
- Python-facing changes update the `pyspart/pyspart.pyi` stubs and the docstrings on the `#[pymethods]` in the same patch.
- User workflow changes should update `README.md`.
- If you detect stale docs while changing related code, fix them in the same patch.

## Review Guidelines (P0/P1 Focus)

Review output should be concise and include only critical issues.

- `P0`: must-fix defects (a query that loses points, a panic reachable from the public API, an unbounded recursion, a broken build, or a broken test
  workflow).
- `P1`: high-priority defects (a violated structural invariant, an entry re-attached at the wrong height, a silently dropped point, a tree that stops
  splitting or subdividing, or a risky change without tests).

Use this review format:

1. `Severity` (`P0`/`P1`)
2. `File:line`
3. `Issue`
4. `Why it matters`
5. `Minimal fix direction`

Do not include style-only feedback or broad praise.
