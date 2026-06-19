use buffa::Message;
use criterion::{black_box, Criterion};

pub trait Serialize: Sized {
    type Message: Default + Into<Self> + Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(
    name: &'static str,
    c: &mut Criterion,
    data: &T,
    access: impl Fn(&[u8]),
    read: impl Fn(&[u8]),
) where
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

    // Zero-copy access via buffa's generated view types. Eager views validate
    // the whole message structure during `decode_view`, so this is the
    // "validated upfront with error" path (it returns a `Result`), mirroring
    // the categorization used for the other pseudo-zero-copy libraries.
    group.bench_function("access (validated upfront with error)", |b| {
        b.iter(|| {
            access(black_box(deserialize_buffer.as_slice()));
            black_box(());
        })
    });

    group.bench_function("read (validated upfront with error)", |b| {
        b.iter(|| {
            read(black_box(deserialize_buffer.as_slice()));
            black_box(());
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
