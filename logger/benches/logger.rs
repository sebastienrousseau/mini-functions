use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate logger;
use logger::*;

fn new_benchmark(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| {
            let log = Log::new(
                "123",
                "2023-01-23 14:04:09.881393 +00:00:00",
                &LogLevel::INFO,
                "test",
                "test log message",
                &LogFormat::COMMON,
            );
            black_box(log);
        })
    });
}
criterion_group!(benches, new_benchmark);
criterion_main!(benches);
