use criterion::{black_box, Criterion};
use databuf::{config::num::LE, *};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + for<'de> Decode<'de>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/databuf", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            serialize_buffer = data.to_bytes::<LE>();
            black_box(serialize_buffer.clone())
        })
    });

    let deserialize_buffer = data.to_bytes::<LE>();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::from_bytes::<LE>(&deserialize_buffer).unwrap());
        })
    });

    crate::bench_size(name, "databuf", deserialize_buffer.as_slice());

    group.finish();
}
