use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use minicbor::{Decode, Encode};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode<()> + for<'de> Decode<'de, ()> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/minicbor", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            minicbor::encode(black_box(data), black_box(serialize_buffer.as_mut_slice())).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    minicbor::encode(&data, &mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(minicbor::decode::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "minicbor", deserialize_buffer.as_slice());

    assert!(minicbor::decode::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode<()> + for<'de> Decode<'de, ()> + BorrowableData,
    for<'a> T::Borrowed<'a>: Encode<()> + Decode<'a, ()>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/minicbor", name));

    let mut deserialize_buffer = Vec::new();
    minicbor::encode(&data, &mut deserialize_buffer).unwrap();
    let bdata = T::Borrowed::from(&data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let mut borrowed_serialize_buffer = Vec::new();
    minicbor::encode(&bdata, &mut borrowed_serialize_buffer).unwrap();
    assert_eq!(borrowed_serialize_buffer, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(
        minicbor::decode::<T::Borrowed<'_>>(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(minicbor::decode::<T::Borrowed<'_>>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    group.finish();
}
