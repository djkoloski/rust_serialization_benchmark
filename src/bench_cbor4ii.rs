use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/cbor4ii", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer = cbor4ii::serde::to_vec(
                std::mem::take(black_box(&mut serialize_buffer)),
                black_box(&data),
            )
            .unwrap();
            black_box(());
        })
    });

    let deserialize_buffer = cbor4ii::serde::to_vec(Vec::new(), &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(cbor4ii::serde::from_slice::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "cbor4ii", deserialize_buffer.as_slice());

    group.finish();
}
