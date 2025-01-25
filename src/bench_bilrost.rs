use crate::datasets::BorrowableData;
use bilrost::buf::ReverseBuffer;
use bilrost::bytes::BufMut;
use bilrost::{BorrowedMessage, Message, OwnedMessage};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: OwnedMessage + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/bilrost", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    let mut prepend_buffer = ReverseBuffer::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.encode(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    group.bench_function("serialize (prepend)", |b| {
        b.iter(|| {
            black_box(&mut prepend_buffer).clear();
            data.prepend(&mut prepend_buffer);
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.encode(&mut deserialize_buffer).unwrap();
    let mut prepended_data = Vec::new();
    prepended_data.put(data.encode_fast());
    // Because there are no unordered collections in the benchmarked types, we can assert that the
    // prepended encoding path emits precisely the same bytes as the forward-encoded one.
    assert_eq!(prepended_data, deserialize_buffer);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::decode(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    crate::bench_size(name, "bilrost", deserialize_buffer.as_slice());

    assert!(T::decode(&*deserialize_buffer).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BorrowableData + OwnedMessage,
    for<'a> T::Borrowed<'a>: BorrowedMessage<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/bilrost", name));

    let deserialize_buffer = data.encode_to_vec();
    let bdata = T::Borrowed::from(&data);

    // The borrowed variant type should encode exactly the same as the owned type.
    assert_eq!(bdata.encode_to_vec(), deserialize_buffer);
    let mut prepended_data = Vec::new();
    prepended_data.put(bdata.encode_fast());
    // Because there are no unordered collections in the benchmarked types, we can assert that
    // the prepended encoding path emits precisely the same bytes as the forward-encoded one.
    assert_eq!(prepended_data, deserialize_buffer);

    // The borrowed value we decode should be equivalent to the input
    assert!(
        T::Borrowed::decode_borrowed(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                T::Borrowed::decode_borrowed(black_box(&deserialize_buffer).as_slice()).unwrap(),
            );
        })
    });

    group.finish();
}
