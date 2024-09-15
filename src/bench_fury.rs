use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};
use fury::{Fury};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/fury", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            Fury::serialize(black_box(serialize_buffer.as_mut_slice()), black_box(&data));
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    Fury::serialize(&mut deserialize_buffer, &data);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(serde_json::from_slice::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "fury", deserialize_buffer.as_slice());

    assert!(Fury::deserialize()::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}
