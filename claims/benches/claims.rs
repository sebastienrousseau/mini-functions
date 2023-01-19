extern crate claims;
use self::claims::Claims;

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::collections::HashMap;

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

fn bench_10_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 10 claims", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

fn bench_100_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 100 claims", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

fn bench_1000_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 1000 claims", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

fn bench_10000_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 10000 claims", |b| {
        b.iter(|| {
            for _ in 0..10000 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

fn bench_100000_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 100000 claims", |b| {
        b.iter(|| {
            for _ in 0..100000 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

fn bench_1000000_claims(c: &mut Criterion) {
    let mut claims: HashMap<String, Claims> = HashMap::new();
    c.bench_function("Benchmarking 1000000 claims", |b| {
        b.iter(|| {
            for _ in 0..1000000 {
                let claim = Claims::default();
                claims.insert(claim.to_string(), claim);
            }
        });
    });
}

criterion_group!(
    benches,
    memory_usage_benchmark,
    set_claim_benchmark,
    get_claim_benchmark,
    remove_claim_benchmark,
    has_claim_benchmark,
    bench_10_claims,
    bench_100_claims,
    bench_1000_claims,
    bench_10000_claims,
    bench_100000_claims,
    bench_1000000_claims
);
criterion_main!(benches);
