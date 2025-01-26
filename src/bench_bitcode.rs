use crate::datasets::BorrowableData;
use bitcode::{Decode, DecodeOwned, Encode};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + DecodeOwned + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::new();

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(buffer.encode(black_box(data)));
        })
    });

    let encoded = buffer.encode(data).to_vec();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(buffer.decode::<T>(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "bitcode", &encoded);

    assert!(buffer.decode::<T>(&encoded).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + DecodeOwned + BorrowableData,
    for<'a> T::Borrowed<'a>: Encode + Decode<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/bitcode", name));
    let mut buffer = bitcode::Buffer::new();

    let encoded = buffer.encode(data).to_vec();
    let bdata = T::Borrowed::from(&data);

    // The borrowed variant type should encode exactly the same as the owned type.
    assert_eq!(bitcode::Buffer::new().encode(&bdata).to_vec(), encoded);

    // The borrowed value we decode should be equivalent to the input
    assert!(buffer.decode::<T::Borrowed<'_>>(&encoded).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(buffer.decode::<T::Borrowed<'_>>(black_box(&encoded)).unwrap());
        })
    });

    group.finish();
}
