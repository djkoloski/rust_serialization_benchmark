use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/nachricht-serde", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            nachricht_serde::to_writer(
                black_box(serialize_buffer.as_mut_slice()),
                black_box(&data),
            )
            .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    nachricht_serde::to_writer(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(nachricht_serde::from_bytes::<T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "nachricht-serde", deserialize_buffer.as_slice());

    assert!(nachricht_serde::from_bytes::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: Serialize + Deserialize<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/nachricht-serde", name));

    let mut deserialize_buffer = Vec::new();
    nachricht_serde::to_writer(&mut deserialize_buffer, &data).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let mut borrowed_buffer = Vec::new();
    nachricht_serde::to_writer(&mut borrowed_buffer, &bdata).unwrap();
    assert_eq!(borrowed_buffer, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(nachricht_serde::from_bytes::<T::Borrowed<'_>>(&deserialize_buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                nachricht_serde::from_bytes::<T::Borrowed<'_>>(black_box(&deserialize_buffer))
                    .unwrap(),
            );
        })
    });

    group.finish();
}
