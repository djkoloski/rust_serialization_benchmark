use bitcode::{Decode, Encode};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::with_capacity(10000000);

    group.bench_function("serialize", |b| {
        b.iter(|| {
            buffer.encode(black_box(&data)).unwrap();
            black_box(());
        })
    });

    let encoded = bitcode::encode(&data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(buffer.decode::<T>(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "bitcode", encoded.as_slice());

    group.finish();
}
