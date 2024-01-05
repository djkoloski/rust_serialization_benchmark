use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: bincode::Encode + bincode::Decode,
{
    const BUFFER_LEN: usize = 10_000_000;
    let mut group = c.benchmark_group(format!("{}/bincode2", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    let conf = bincode::config::standard();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            bincode::encode_into_slice(
                black_box(&data),
                black_box(serialize_buffer.as_mut_slice()),
                conf,
            )
            .unwrap();
            black_box(());
        })
    });

    let deserialize_buffer = serialize_buffer.as_mut_slice();
    let size = bincode::encode_into_slice(&data, deserialize_buffer, conf).unwrap();
    let deserialize_buffer = &deserialize_buffer[..size];

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::decode_from_slice::<T, _>(black_box(deserialize_buffer), conf)
                    .unwrap()
                    .0,
            );
        })
    });

    crate::bench_size(name, "bincode2", deserialize_buffer);

    group.finish();
}
