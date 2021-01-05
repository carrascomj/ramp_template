use criterion::{criterion_group, criterion_main, Criterion};

extern crate {{crate_name}};

// Application code to benchmark
fn run_perf_code() {

}

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("Run a function as benchmark", |b| {
        b.iter(run_benchmark1)
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
