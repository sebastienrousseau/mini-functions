extern crate criterion;
extern crate random;
use self::random::Random;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_random(c: &mut Criterion) {
    // Benchmark the random bool function
    c.bench_function("Random bool", |b| {
        b.iter(|| Random::bool(&mut Random::new(), black_box(0.5)))
    });

    // Benchmark the random bytes function
    c.bench_function("Random bytes", |b| {
        b.iter(|| Random::bytes(&mut Random::new(), black_box(1000)))
    });

    // Benchmark the random char function
    c.bench_function("Random char", |b| {
        b.iter(|| Random::char(&mut Random::new()))
    });

    // Benchmark the random choose function
    c.bench_function("Random choose", |b| {
        b.iter(|| {
            let mut rng = Random::new();
            let values = vec![1, 2, 3, 4, 5];
            Random::choose(&mut rng, &values);
        })
    });

    // Benchmark the random float function
    c.bench_function("Random float", |b| {
        b.iter(|| Random::float(&mut Random::new()))
    });

    // Benchmark the random int function
    c.bench_function("Random int", |b| {
        b.iter(|| Random::int(&mut Random::new(), black_box(0), black_box(100)))
    });

    // Benchmark the random new function
    c.bench_function("Random new", |b| b.iter(|| Random::new));

    // Benchmark the random pseudo function
    c.bench_function("Random pseudo", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.pseudo())
    });

    // Benchmark the random function
    c.bench_function("Random random", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.rand())
    });

    // Benchmark the random range function
    c.bench_function("Random range", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.range(black_box(0), black_box(100)))
    });
}

criterion_group!(benches, benchmark_random);
criterion_main!(benches);
