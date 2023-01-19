extern crate claims;
use self::claims::Claims;

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn memory_usage_benchmark(c: &mut Criterion) {
    c.bench_function("memory_usage", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            let memory_usage = std::mem::size_of_val(&claims);
            black_box(memory_usage);
        });
    });
}
fn set_claim_benchmark(c: &mut Criterion) {
    c.bench_function("set_claim", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";

        b.iter(|| {
            claims.set_claim(key, value);
        });
    });
}

fn get_claim_benchmark(c: &mut Criterion) {
    c.bench_function("get_claim", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.get_claim(key);
        });
    });
}

fn remove_claim_benchmark(c: &mut Criterion) {
    c.bench_function("remove_claim", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.remove_claim(key);
        });
    });
}

fn has_claim_benchmark(c: &mut Criterion) {
    c.bench_function("has_claim", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.has_claim(key);
        });
    });
}

fn clear_claims_benchmark(c: &mut Criterion) {
    c.bench_function("clear_claims", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.clear_claims();
        });
    });
}

fn len_benchmark(c: &mut Criterion) {
    c.bench_function("len", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.len();
        });
    });
}

fn is_empty_benchmark(c: &mut Criterion) {
    c.bench_function("is_empty", |b| {
        let mut claims = Claims::new();
        let key = "key";
        let value = "value";
        claims.set_claim(key, value);

        b.iter(|| {
            claims.is_empty();
        });
    });
}

fn scale_claim_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_claim");
    for i in [10, 100, 1000, 10000, 100000].iter() {
        group.bench_with_input(format!("{} claims", i), i, |b, i| {
            let mut claims = Claims::new();
            let key = "key";
            let value = "value";
            b.iter(|| {
                for _ in 0..*i {
                    claims.set_claim(key, value);
                }
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    clear_claims_benchmark,
    get_claim_benchmark,
    has_claim_benchmark,
    is_empty_benchmark,
    len_benchmark,
    memory_usage_benchmark,
    remove_claim_benchmark,
    scale_claim_benchmark,
    set_claim_benchmark,
);
criterion_main!(benches);
