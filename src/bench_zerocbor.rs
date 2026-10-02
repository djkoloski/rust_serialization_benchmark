use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use zerocbor::{FromCbor, ToCbor};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: ToCbor + for<'de> FromCbor<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;
    let mut group = c.benchmark_group(format!("{}/zerocbor", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            zerocbor::to_cbor(black_box(data), black_box(&mut serialize_buffer)).unwrap();
            black_box(())
        })
    });

    let deserialize_buffer = zerocbor::to_cbor_vec(data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(zerocbor::from_cbor::<T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "zerocbor", deserialize_buffer.as_slice());

    assert!(zerocbor::from_cbor::<T>(black_box(&deserialize_buffer)).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: ToCbor + for<'de> FromCbor<'de> + BorrowableData,
    for<'a> T::Borrowed<'a>: ToCbor + FromCbor<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/zerocbor", name));

    let deserialize_buffer = zerocbor::to_cbor_vec(data).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let borrowed_buffer = zerocbor::to_cbor_vec(&bdata).unwrap();
    assert_eq!(borrowed_buffer, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(zerocbor::from_cbor::<T::Borrowed<'_>>(&deserialize_buffer).unwrap() == bdata);

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                zerocbor::from_cbor::<T::Borrowed<'_>>(black_box(&deserialize_buffer)).unwrap(),
            );
        })
    });

    group.finish();
}
