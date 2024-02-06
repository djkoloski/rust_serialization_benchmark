use criterion::{black_box, Criterion};
use prost::Message;

pub trait Serialize: Sized {
    type Message: Default + Into<Self> + Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/prost", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.serialize_pb().encode(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message.encode(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize_pb().encode(&mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(<T::Message>::decode(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    crate::bench_size(name, "prost", deserialize_buffer.as_slice());

    assert!(&<T::Message>::decode(&*deserialize_buffer).unwrap().into() == data);

    group.finish();
}
