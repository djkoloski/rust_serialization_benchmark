use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};

const BUFFER_LEN: usize = 10_000_000;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: bincode::Encode + bincode::Decode<()> + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/bincode", name));

    let mut buffer = vec![0u8; BUFFER_LEN];
    let conf = bincode::config::standard();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let size = bincode::encode_into_slice(black_box(&data), black_box(&mut *buffer), conf)
                .unwrap();
            black_box(&buffer[..size]);
        })
    });

    let size = bincode::encode_into_slice(data, &mut *buffer, conf).unwrap();
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

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: bincode::Encode + for<'de> bincode::Decode<()> + BorrowableData,
    for<'a> T::Borrowed<'a>: bincode::Encode + bincode::BorrowDecode<'a, ()>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/bincode", name));

    let mut deserialize_buffer = vec![0u8; BUFFER_LEN];
    let conf = bincode::config::standard();
    let size = bincode::encode_into_slice(data, &mut deserialize_buffer, conf).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let mut borrowed_serialized = vec![0u8; BUFFER_LEN];
    bincode::encode_into_slice(&bdata, &mut borrowed_serialized, conf).unwrap();
    assert_eq!(borrowed_serialized, deserialize_buffer);

    let buffer = &deserialize_buffer[..size];

    // The borrowed value we decode should be equivalent to the input
    assert!(
        bincode::borrow_decode_from_slice::<T::Borrowed<'_>, _>(buffer, conf)
            .unwrap()
            .0
            == bdata
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                bincode::borrow_decode_from_slice::<T::Borrowed<'_>, _>(black_box(buffer), conf)
                    .unwrap(),
            );
        })
    });

    group.finish();
}
