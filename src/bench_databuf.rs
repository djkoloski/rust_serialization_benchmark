use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use databuf::{config::num::LE, *};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + for<'de> Decode<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/databuf", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            data.encode::<LE>(&mut serialize_buffer).unwrap();
        })
    });

    let deserialize_buffer = data.to_bytes::<LE>();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::from_bytes::<LE>(&deserialize_buffer).unwrap());
        })
    });

    crate::bench_size(name, "databuf", deserialize_buffer.as_slice());

    assert!(T::from_bytes::<LE>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + for<'de> Decode<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: Encode + Decode<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/databuf", name));

    let deserialize_buffer = data.to_bytes::<LE>();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    assert_eq!(bdata.to_bytes::<LE>(), deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(T::Borrowed::from_bytes::<LE>(&deserialize_buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(T::Borrowed::from_bytes::<LE>(&deserialize_buffer).unwrap());
        })
    });

    group.finish();
}
