use blake3::Hasher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hsh::Hash;

fn generate_hash(c: &mut Criterion) {
    c.bench_function("generate_hash", |b| {
        let mut hasher = Hasher::new();
        let password = "password";
        b.iter(|| {
            hasher.update(password.as_bytes());
            hasher.finalize();
        });
    });
}

fn hash_entropy(c: &mut Criterion) {
    let mut hash_struct = Hash::new();
    hash_struct.set_password("password");
    hash_struct.set_hash("abcdefghijklmnopqrstuvwxyz");
    c.bench_function("hash_entropy", |b| {
        b.iter(|| {
            black_box(&hash_struct).entropy();
        });
    });
}

fn verify_password(c: &mut Criterion) {
    let mut hash_struct = Hash::new();
    hash_struct.set_password("password");
    hash_struct.set_hash("abcdefghijklmnopqrstuvwxyz");
    c.bench_function("verify_password", |b| {
        let hash = "abcdefghijklmnopqrstuvwxyz";
        let password = "password";
        b.iter(|| {
            black_box(hash_struct.verify(hash, password));
        });
    });
}

fn bench_generate_hash(c: &mut Criterion) {
    c.bench_function("generate_hash", |b| {
        let password = "password";
        let mut hash_struct = Hash::new();
        hash_struct.set_password(password);
        b.iter(|| {
            hash_struct.generate_hash();
        });
    });
}

fn bench_hash(c: &mut Criterion) {
    c.bench_function("hash", |b| {
        let mut hash_struct = Hash::new();
        hash_struct.set_password("password");
        hash_struct.set_hash("abcdefghijklmnopqrstuvwxyz");
        b.iter(|| {
            black_box(hash_struct.hash());
        });
    });
}

fn bench_hash_length(c: &mut Criterion) {
    c.bench_function("hash_length", |b| {
        let mut hash_struct = Hash::new();
        hash_struct.set_password("password");
        hash_struct.set_hash("abcdefghijklmnopqrstuvwxyz");
        b.iter(|| {
            black_box(hash_struct.hash_length());
        });
    });
}

fn bench_new(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| {
            Hash::new();
        });
    });
}

fn bench_password(c: &mut Criterion) {
    c.bench_function("password", |b| {
        let mut hash_struct = Hash::new();
        hash_struct.set_password("password");
        b.iter(|| {
            black_box(hash_struct.password());
        });
    });
}

fn bench_password_length(c: &mut Criterion) {
    c.bench_function("password_length", |b| {
        let mut hash_struct = Hash::new();
        hash_struct.set_password("password");
        b.iter(|| {
            black_box(hash_struct.password_length());
        });
    });
}

fn bench_set_hash(c: &mut Criterion) {
    c.bench_function("set_hash", |b| {
        let mut hash_struct = Hash::new();
        let hash = "abcdefghijklmnopqrstuvwxyz";
        b.iter(|| {
            hash_struct.set_hash(hash);
        });
    });
}

fn bench_set_password(c: &mut Criterion) {
    c.bench_function("set_password", |b| {
        let mut hash_struct = Hash::new();
        let password = "password";
        b.iter(|| {
            hash_struct.set_password(password);
        });
    });
}

fn bench_verify(c: &mut Criterion) {
    c.bench_function("verify", |b| {
        let mut hash_struct = Hash::new();
        hash_struct.set_password("password");
        hash_struct.set_hash("abcdefghijklmnopqrstuvwxyz");
        let hash = "abcdefghijklmnopqrstuvwxyz";
        let password = "password";
        b.iter(|| {
            black_box(hash_struct.verify(hash, password));
        });
    });
}

criterion_group!(
    benches,
    bench_generate_hash,
    bench_hash,
    bench_hash_length,
    bench_new,
    bench_password,
    bench_password_length,
    bench_set_hash,
    bench_set_password,
    bench_verify,
    generate_hash,
    hash_entropy,
    verify_password
);
criterion_main!(benches);
