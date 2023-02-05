use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::Rgba;
extern crate qrc;
use self::qrc::QRCode;

// Benchmark for QRCode::new
fn new_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::new", |b| {
        b.iter(|| QRCode::new(black_box(vec![1, 2, 3])))
    });
}
// Benchmark for QRCode::to_png
fn to_png_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::to_png", |b| b.iter(|| qrcode.to_png(512)));
}

// Benchmark for QRCode::from_string
fn from_string_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::from_string", |b| {
        b.iter(|| QRCode::from_string(black_box("Hello, world!".to_string())))
    });
}

// Benchmark for QRCode::from_bytes
fn from_bytes_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::from_bytes", |b| {
        b.iter(|| QRCode::from_bytes(black_box(vec![1, 2, 3])))
    });
}

// Benchmark for QRCode::to_svg
fn to_svg_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::to_svg", |b| {
        b.iter(|| qrcode.to_svg(black_box(100)))
    });
}

// Benchmark for QRCode::colorize
fn colorize_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    let color = Rgba([0, 0, 0, 0]);
    c.bench_function("QRCode::colorize", |b| {
        b.iter(|| qrcode.colorize(black_box(color)))
    });
}

// Benchmark for QRCode::resize
fn resize_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::resize", |b| {
        b.iter(|| qrcode.resize(black_box(100), black_box(100)))
    });
}

criterion_group!(
    benches,
    colorize_benchmark,
    from_bytes_benchmark,
    from_string_benchmark,
    new_benchmark,
    resize_benchmark,
    to_png_benchmark,
    to_svg_benchmark,
);
criterion_main!(benches);
