use core::fmt;
use criterion::{black_box, Criterion};
use msgpacker::{Encoder, EncoderSlice, Packable, Unpackable, UnpackableBorrowed};

use crate::datasets::BorrowableData;

const BUFFER_LEN: usize = 10_000_000;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Packable + Unpackable + PartialEq,
    <T as Unpackable>::Error: fmt::Debug,
{
    let mut group = c.benchmark_group(format!("{}/msgpacker", name));
    let mut encoder = Encoder::with_capacity(BUFFER_LEN);
    T::pack(data, &mut encoder);
    let mut buffer = encoder.into_inner();
    let mut encoder = EncoderSlice::new(&mut buffer);

    group.bench_function("serialize", |b| {
        b.iter(|| {
            T::pack(black_box(data), black_box(&mut encoder));
            encoder.reset();
        });
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            T::unpack(black_box(&buffer)).unwrap();
        })
    });

    crate::bench_size(name, "msgpacker", &buffer);

    // sanity check
    assert!(T::unpack(&buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Packable + Unpackable + PartialEq + BorrowableData,
    <T as Unpackable>::Error: fmt::Debug,
    for<'a> T::Borrowed<'a>: Packable + UnpackableBorrowed<'a>,
    for<'a> <T::Borrowed<'a> as UnpackableBorrowed<'a>>::Error: fmt::Debug,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/msgpacker", name));
    let mut encoder = Encoder::with_capacity(BUFFER_LEN);
    T::pack(data, &mut encoder);
    let buffer = encoder.into_inner();

    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let borrowed_serialized = bdata.pack_to_vec();
    assert_eq!(borrowed_serialized, buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(T::Borrowed::unpack(&buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(T::Borrowed::unpack(black_box(&buffer)).unwrap());
        })
    });

    group.finish();
}
