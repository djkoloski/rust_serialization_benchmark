use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/serde-zap", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                serde_zap::to_slice(black_box(&data), black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    let deserialize_buffer = serde_zap::to_vec(&data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(serde_zap::from_bytes::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "serde-zap", deserialize_buffer.as_slice());

    assert!(serde_zap::from_bytes::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: Serialize + Deserialize<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/serde-zap", name));

    let deserialize_buffer = serde_zap::to_vec(&data).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    assert_eq!(serde_zap::to_vec(&bdata).unwrap(), deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(serde_zap::from_bytes::<T::Borrowed<'_>>(&deserialize_buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                serde_zap::from_bytes::<T::Borrowed<'_>>(black_box(&deserialize_buffer)).unwrap(),
            );
        })
    });

    group.finish();
}
