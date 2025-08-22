use criterion::criterion_main;

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
