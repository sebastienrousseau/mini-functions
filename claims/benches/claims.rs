extern crate claims;
use self::claims::Claims;

extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

use std::collections::HashMap;

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

criterion_group!(benches, bench_1000_claims);
criterion_main!(benches);
