use crate::datasets::BorrowableData;
use criterion::{black_box, Criterion};
use speedy::{Endianness, Readable, Writable};

#[cfg(target_endian = "little")]
const CONTEXT: Endianness = Endianness::LittleEndian;
#[cfg(target_endian = "big")]
const CONTEXT: Endianness = Endianness::BigEndian;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: for<'a> Readable<'a, Endianness> + Writable<Endianness> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/speedy", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            data.write_to_buffer_with_ctx(CONTEXT, black_box(serialize_buffer.as_mut_slice()))
                .unwrap();
            black_box(());
        })
    });

    let deserialize_buffer = data.write_to_vec_with_ctx(CONTEXT).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                T::read_from_buffer_with_ctx(CONTEXT, black_box(deserialize_buffer.as_slice()))
                    .unwrap(),
            );
        })
    });

    crate::bench_size(name, "speedy", deserialize_buffer.as_slice());

    assert!(T::read_from_buffer_with_ctx(CONTEXT, &deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: for<'a> Readable<'a, Endianness> + Writable<Endianness> + BorrowableData,
    for<'a> T::Borrowed<'a>: Readable<'a, Endianness> + Writable<Endianness>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/speedy", name));

    let deserialize_buffer = data.write_to_vec_with_ctx(CONTEXT).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    assert_eq!(
        bdata.write_to_vec_with_ctx(CONTEXT).unwrap(),
        deserialize_buffer
    );

    // The borrowed value we decode should be equivalent to the input
    assert!(
        T::Borrowed::read_from_buffer_with_ctx(CONTEXT, deserialize_buffer.as_slice()).unwrap()
            == bdata
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                T::Borrowed::read_from_buffer_with_ctx(
                    CONTEXT,
                    black_box(deserialize_buffer.as_slice()),
                )
                .unwrap(),
            );
        })
    });

    group.finish();
}
