use criterion::criterion_main;

mod delete_benchmarks;
mod insert_benchmarks;
mod knn_search_benchmarks;
mod range_search_benchmarks;

// Main entry point for running the benchmarks
criterion_main!(
    delete_benchmarks::benches,
    insert_benchmarks::benches,
    knn_search_benchmarks::benches,
    range_search_benchmarks::benches
);
