use cmn::constants::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psph::Password;

fn new_benchmark(c: &mut Criterion) {
    c.bench_function("Password::new", |b| {
        b.iter(|| {
            Password::new(
                black_box(4),
                black_box("-"),
                black_box(SPECIAL_CHARS.to_vec()),
            )
        })
    });
}
criterion_group!(benches, new_benchmark);
criterion_main!(benches);
