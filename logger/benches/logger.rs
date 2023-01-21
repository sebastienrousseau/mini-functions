use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate logger;
use logger::*;

fn new_benchmark(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| {
            let log = Log::new(
                black_box("session_id"),
                black_box("time"),
                black_box(&LogLevel::ALL),
                black_box("component"),
                black_box("description"),
            );
            black_box(log);
        })
    });
}
criterion_group!(benches, new_benchmark);
criterion_main!(benches);
