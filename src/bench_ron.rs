use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/ron", name));

    let mut serialize_buffer = String::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            ron::ser::to_writer(black_box(&mut serialize_buffer), black_box(&data))
                .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = String::new();
    ron::ser::to_writer(&mut deserialize_buffer, &data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(ron::de::from_bytes::<'_, T>(black_box(deserialize_buffer.as_ref())).unwrap());
        })
    });

    crate::bench_size(name, "ron", deserialize_buffer.as_ref());

    assert!(ron::de::from_bytes::<T>(deserialize_buffer.as_ref()).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: Serialize + Deserialize<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/ron", name));

    let mut deserialize_buffer = String::new();
    ron::ser::to_writer(&mut deserialize_buffer, &data).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let mut borrowed_buffer = String::new();
    ron::ser::to_writer(&mut borrowed_buffer, &bdata).unwrap();
    assert_eq!(borrowed_buffer, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(ron::de::from_bytes::<T::Borrowed<'_>>(deserialize_buffer.as_ref()).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                ron::de::from_bytes::<T::Borrowed<'_>>(black_box(deserialize_buffer.as_ref())).unwrap(),
            );
        })
    });

    group.finish();
}
