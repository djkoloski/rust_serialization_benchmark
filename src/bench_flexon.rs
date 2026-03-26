use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 30_000_000;

    let mut group = c.benchmark_group(format!("{name}/flexon"));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            flexon::to_writer(black_box(&mut serialize_buffer), black_box(data)).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    flexon::to_writer(&mut deserialize_buffer, data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(flexon::from_slice::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "flexon", &deserialize_buffer);

    assert!(flexon::from_slice::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}
