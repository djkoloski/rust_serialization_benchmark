use buffa::{HasMessageView, Message, MessageView};
use criterion::{black_box, Criterion};

pub trait Serialize: Sized {
    type Message: Default + Into<Self> + Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/buffa", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.serialize_pb().encode(&mut serialize_buffer);
            black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message.encode(&mut serialize_buffer);
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize_pb().encode(&mut deserialize_buffer);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(<T::Message>::decode_from_slice(black_box(&deserialize_buffer)).unwrap());
        })
    });

    group.bench_function("deserialize (decode + convert)", |b| {
        b.iter(|| {
            let decoded = <T::Message>::decode_from_slice(black_box(&deserialize_buffer)).unwrap();
            black_box(decoded.into());
        })
    });

    crate::bench_size(name, "buffa", deserialize_buffer.as_slice());

    assert!(
        <T::Message>::decode_from_slice(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.finish();
}

// In addition to the owned `bench`, this also benchmarks decoding into buffa's
// generated borrowed view type, which references string/bytes data in the input
// buffer instead of allocating owned copies. This is buffa's "borrow" target;
// it still walks the whole message, so it is not a zero-copy access/read.
//
// The view type is reached generically through `HasMessageView` on the owned
// message, so callers don't need to name it.
pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
    T::Message: HasMessageView,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/buffa", name));

    let mut deserialize_buffer = Vec::new();
    data.serialize_pb().encode(&mut deserialize_buffer);

    // The borrowed view should decode to the same data as the owned value.
    assert!(
        <T::Message as HasMessageView>::View::decode_view(&deserialize_buffer)
            .unwrap()
            .to_owned_message()
            .into()
            == *data
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                <T::Message as HasMessageView>::View::decode_view(black_box(&deserialize_buffer))
                    .unwrap(),
            );
        })
    });

    group.finish();
}
