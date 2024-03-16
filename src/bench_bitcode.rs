use bitcode::{DecodeOwned, Encode};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + DecodeOwned + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::EncodeBuffer::<T>::default();

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(buffer.encode(black_box(data)));
        })
    });

    let encoded = buffer.encode(data);
    let mut buffer = bitcode::DecodeBuffer::<T>::default();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(buffer.decode(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "bitcode", encoded);

    assert!(buffer.decode(&encoded).unwrap() == *data);

    group.finish();
}
