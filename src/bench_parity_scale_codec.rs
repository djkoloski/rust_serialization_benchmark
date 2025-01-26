use criterion::{black_box, Criterion};
use parity_scale_codec::{Decode, Encode};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/parity-scale-codec", name));

    let mut serialize_buffer = vec![0u8; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            data.encode_to(black_box(&mut serialize_buffer.as_mut_slice()));
            black_box(());
        })
    });

    let deserialize_buffer = data.encode();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::decode(&mut deserialize_buffer.as_slice()).unwrap());
        })
    });

    crate::bench_size(name, "parity-scale-codec", deserialize_buffer.as_slice());

    assert!(T::decode(&mut deserialize_buffer.as_slice()).unwrap() == *data);

    group.finish();
}

// parity_scale_codec does not appear to support borrowed decoding.
