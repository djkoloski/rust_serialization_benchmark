use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: bincode::Encode + bincode::Decode + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;
    let mut group = c.benchmark_group(format!("{}/bincode", name));

    let mut buffer = Box::new([0u8; BUFFER_LEN]);
    let conf = bincode::config::standard();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let size = bincode::encode_into_slice(black_box(&data), black_box(&mut *buffer), conf)
                .unwrap();
            black_box(&buffer[..size]);
        })
    });

    let size = bincode::encode_into_slice(&data, &mut *buffer, conf).unwrap();
    let buffer = &buffer[..size];

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::decode_from_slice::<T, _>(black_box(buffer), conf)
                    .unwrap()
                    .0,
            );
        })
    });

    crate::bench_size(name, "bincode", buffer);

    assert!(
        bincode::decode_from_slice::<T, _>(black_box(buffer), conf)
            .unwrap()
            .0
            == *data
    );

    group.finish();
}
