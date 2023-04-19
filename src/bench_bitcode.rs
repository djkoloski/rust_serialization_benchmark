use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::with_capacity(1000000);

    group.bench_function("serialize", |b| {
        b.iter(|| {
            buffer.serialize(black_box(&data))
                .unwrap();
            black_box(());
        })
    });

    let encoded = bitcode::serialize(&data).unwrap();
    let mut buffer = bitcode::Buffer::with_capacity(1000000);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(buffer.deserialize::<T>(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "bitcode", encoded.as_slice());

    group.finish();
}
