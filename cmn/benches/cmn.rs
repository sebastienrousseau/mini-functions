use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use cmn::Constants;
pub use cmn::Words;

pub struct Common;

impl Common {
    pub fn new() -> Self {
        Common
    }
    pub fn constants(&self) -> Constants {
        Constants
    }
    pub fn words(&self) -> Words {
        Words::new()
    }
}

impl Default for Common {
    fn default() -> Self {
        Self::new()
    }
}

fn bench_common(c: &mut Criterion) {
    c.bench_function("common", |b| {
        b.iter(|| {
            let common = black_box(Common::default());
            black_box(common.constants());
            black_box(common.words());
        });
    });
}

criterion_group!(benches, bench_common);
criterion_main!(benches);
