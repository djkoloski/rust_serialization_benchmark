use serde::{Deserialize, Serialize};
use criterion::{black_box, Criterion};

pub fn bench<T, F>(c: &mut Criterion, data: &T, update: F)
where
    T: Serialize + for<'de> Deserialize<'de>,
    F: Fn(&mut T),
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group("serde_json");

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                serde_json::to_writer(
                    black_box(serialize_buffer.as_mut_slice()),
                    black_box(&data),
                )
                .unwrap()
            );
        })
    });

    let mut deserialize_buffer = Vec::new();
    serde_json::to_writer(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("update", |b| {
        b.iter(|| {
            let mut value = serde_json::from_slice::<'_, T>(black_box(&deserialize_buffer))
                .unwrap();
            update(&mut value);
            let result = serde_json::to_writer(
                black_box(serialize_buffer.as_mut_slice()),
                black_box(&value),
            ).unwrap();
            black_box(result);
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                serde_json::from_slice::<'_, T>(black_box(&deserialize_buffer))
                .unwrap()
            );
        })
    });

    println!("serde_json size: {} bytes", deserialize_buffer.len());

    group.finish();
}