use criterion::{black_box, BatchSize, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/simd-json", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                simd_json::serde::to_writer(
                    black_box(serialize_buffer.as_mut_slice()),
                    black_box(&data),
                )
                .unwrap(),
            );
        })
    });

    let mut deserialize_buffer = Vec::new();
    simd_json::serde::to_writer(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter_batched_ref(
            || deserialize_buffer.clone(),
            |deserialize_buffer| {
                black_box(
                    simd_json::serde::from_slice::<'_, T>(deserialize_buffer.as_mut_slice())
                        .unwrap(),
                );
            },
            BatchSize::SmallInput,
        )
    });

    crate::bench_size(name, "simd-json", deserialize_buffer.as_slice());

    group.finish();
}
