use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 70_000_000;

    let mut group = c.benchmark_group(format!("{}/pot", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                pot::to_writer(black_box(&data), black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    let deserialize_buffer = pot::to_vec(&data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(pot::from_slice::<T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "pot", deserialize_buffer.as_slice());

    group.finish();
}
