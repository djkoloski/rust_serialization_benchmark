use serde::{Deserialize, Serialize};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/ron", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                ron::ser::to_writer(
                    black_box(serialize_buffer.as_mut_slice()),
                    black_box(&data),
                )
                .unwrap()
            );
        })
    });

    let mut deserialize_buffer = Vec::new();
    ron::ser::to_writer(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                ron::de::from_bytes::<'_, T>(black_box(&deserialize_buffer))
                .unwrap()
            );
        })
    });

    println!("{}/ron/size {}", name, deserialize_buffer.len());
    println!("{}/ron/zlib {}", name, crate::zlib_size(deserialize_buffer.as_slice()));

    group.finish();
}