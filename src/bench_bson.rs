use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/bson", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            let mut target_buffer = Vec::new();
            ::core::mem::swap(&mut serialize_buffer, &mut target_buffer);
            let mut target_buffer =
                black_box(bson::ser::into_vec(black_box(&data), black_box(target_buffer)).unwrap());
            ::core::mem::swap(&mut serialize_buffer, &mut target_buffer);
        })
    });

    let deserialize_buffer = bson::ser::into_vec(&data, Vec::new()).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(bson::from_slice::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "cbor", deserialize_buffer.as_slice());

    group.finish();
}
