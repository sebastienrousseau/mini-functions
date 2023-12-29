extern crate criterion;
use cclm::Claims;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate cjwt;
use self::cjwt::{Algorithm, Header, JWT};

fn bench_to_string_benchmark(c: &mut Criterion) {
    let jwt = JWT::default();

    c.bench_function("to_string", move |b| b.iter(|| jwt.to_string()));
}

fn bench_default_benchmark(c: &mut Criterion) {
    c.bench_function("default", |b| b.iter(|| JWT::default));
}

fn bench_decode_benchmark(c: &mut Criterion) {
    let secret: &[u8; 6] = b"secret";
    let header = Header::default();
    let claims = Claims::default();

    let token = JWT::encode(header, claims, secret).unwrap();
    let mut jwt =
        JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token,
        };

    c.bench_function("decode", move |b| b.iter(|| jwt.decode(secret)));
}

fn bench_generate_benchmark(c: &mut Criterion) {
    let secret = b"secret";
    c.bench_function("generate", |b| {
        b.iter(|| JWT::generate(black_box(secret)).unwrap())
    });
}

// fn bench_generate_benchmark(c: &mut Criterion) {
//     c.bench_function("generate", |b| b.iter(|| JWT::generate()));
// }

fn bench_get_token_benchmark(c: &mut Criterion) {
    let jwt = JWT {
        header: Header::default(),
        claims: Claims::default(),
        signature: vec![],
        token: "example_token".to_owned(),
    };
    let result = JWT::get_token(jwt);
    c.bench_function("get_token", move |b| b.iter(|| result.clone()));
}

fn bench_claims_benchmark(c: &mut Criterion) {
    c.bench_function("claims", move |b| b.iter(|| JWT::claims));
}

fn bench_get_token_length_benchmark(c: &mut Criterion) {
    let jwt = JWT {
        header: Header::default(),
        claims: Claims::default(),
        signature: vec![],
        token: "example_token".to_owned(),
    };
    let result = JWT::get_token_length(jwt);
    c.bench_function("get_token_length", move |b| b.iter(|| result));
}

fn bench_get_token_header_benchmark(c: &mut Criterion) {
    c.bench_function("get_token_header", move |b| {
        let jwt = JWT {
            header: Header {
                alg: Some(Algorithm::HS256),
                kid: Some("example_kid".to_string()),
                typ: Some("example_type".to_string()),
                cty: Some("example_cty".to_string()),
            },
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = JWT::get_token_header(jwt);
        b.iter(|| result.clone())
    });
}

fn bench_encode_benchmark(c: &mut Criterion) {
    let secret = b"secret";
    let header = JWT::default().header;
    let claims = JWT::default().claims;

    c.bench_function("encode", move |b| {
        b.iter(|| {
            JWT::encode(
                black_box(header.clone()),
                black_box(claims.clone()),
                black_box(secret),
            )
        })
    });
}
fn bench_validate_benchmark(c: &mut Criterion) {
    let secret = b"secret";
    let jwt = JWT {
        header: Header {
            alg: Some(Algorithm::HS256),
            kid: Some("example_kid".to_string()),
            typ: Some("example_type".to_string()),
            cty: Some("example_cty".to_string()),
        },
        claims: Claims::default(),
        signature: vec![],
        token: "example_token".to_owned(),
    };

    c.bench_function("validate", move |b| {
        b.iter(|| JWT::validate(black_box(&jwt), black_box(secret)));
    });
}

criterion_group!(
    benches,
    bench_claims_benchmark,
    bench_decode_benchmark,
    bench_default_benchmark,
    bench_encode_benchmark,
    bench_generate_benchmark,
    bench_get_token_benchmark,
    bench_get_token_header_benchmark,
    bench_get_token_length_benchmark,
    bench_to_string_benchmark,
    bench_validate_benchmark,
);
criterion_main!(benches);
