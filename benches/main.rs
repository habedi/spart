use criterion::criterion_main;

// Declared once here so the bench modules below share a single copy; each of them declaring its own
// `#[path = "shared.rs"] mod shared;` compiled the same file five times over.
mod shared;

mod bench_delete;
mod bench_insert;
mod bench_insert_bulk;
mod bench_knn_search;
mod bench_range_search;
mod bench_serialization;

// Main entry point for running the benchmarks
criterion_main!(
    bench_delete::benches,
    bench_insert::benches,
    bench_insert_bulk::benches,
    bench_knn_search::benches,
    bench_range_search::benches,
    bench_serialization::benches
);
