use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate md5;
use md5::*;

pub const BLOCK_LENGTH: usize = 64;

fn finalize_benchmark(c: &mut Criterion) {
    c.bench_function("finalize", |b| {
        let mut md5 = MD5::new();
        let data = vec![0u8; BLOCK_LENGTH];
        md5.update(&data);

        b.iter(|| {
            md5.finalize();
            // md5.digest;
        });
    });
}

fn new_benchmark(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| MD5::new);
    });
}

fn transform_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();
    let data = vec![0u8; BLOCK_LENGTH];

    c.bench_function("transform", |b| {
        b.iter(|| {
            md5.transform(black_box(&data));
        });
    });
}

fn update_with_len_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();
    let data = vec![0u8; BLOCK_LENGTH];
    let nbytes = Some(BLOCK_LENGTH);

    c.bench_function("update_with_len", |b| {
        b.iter(|| {
            // md5.update_with_len(black_box(&data), black_box(nbytes));
            md5.update_with_len(black_box(&data), black_box(nbytes.unwrap_or(BLOCK_LENGTH)));
        });
    });
}

fn reset_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();

    c.bench_function("reset", |b| {
        b.iter(|| {
            md5.reset();
        });
    });
}

fn update_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();
    let data = vec![0u8; BLOCK_LENGTH];

    c.bench_function("update", |b| {
        b.iter(|| {
            md5.update(black_box(&data));
        });
    });
}

fn update_file_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();
    let path = "../file.txt";

    c.bench_function("update_file", |b| {
        b.iter(|| {
            md5.update_file(black_box(path));
        });
    });
}

fn hexdigest_benchmark(c: &mut Criterion) {
    let data = vec![0u8; BLOCK_LENGTH];

    c.bench_function("hexdigest", |b| {
        b.iter(|| {
            let data_str = String::from_utf8(data.to_vec()).unwrap();
            MD5::hexdigest(black_box(&data_str));
        });
    });
}

fn hexdigest_file_benchmark(c: &mut Criterion) {
    let path = "../file.txt";

    c.bench_function("hexdigest_file", |b| {
        b.iter(|| {
            MD5::hexdigest_file(black_box(path));
        });
    });
}

fn reset_file_benchmark(c: &mut Criterion) {
    let mut md5 = MD5::new();
    let path = "../file.txt";

    c.bench_function("reset_file", |b| {
        b.iter(|| {
            md5.reset_file(black_box(path));
        });
    });
}

criterion_group!(
    benches,
    finalize_benchmark,
    hexdigest_benchmark,
    hexdigest_file_benchmark,
    new_benchmark,
    reset_benchmark,
    reset_file_benchmark,
    transform_benchmark,
    update_benchmark,
    update_file_benchmark,
    update_with_len_benchmark,
);
criterion_main!(benches);
