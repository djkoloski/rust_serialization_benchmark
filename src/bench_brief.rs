use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/brief", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                brief::to_slice(black_box(&data), black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    let deserialize_buffer = brief::to_vec(&data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(brief::from_slice::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "brief", deserialize_buffer.as_slice());

    assert!(brief::from_slice::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}
