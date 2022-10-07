use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/rmp", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            rmp_serde::encode::write(
                &mut black_box(serialize_buffer.as_mut_slice()),
                black_box(&data),
            )
            .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    rmp_serde::encode::write(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(rmp_serde::from_slice::<T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "rmp", deserialize_buffer.as_slice());

    group.finish();
}
