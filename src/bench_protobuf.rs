use criterion::{black_box, Criterion};
use protobuf::Message;

pub trait Serialize: Sized {
    type Message: Default + Into<Self> + Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/protobuf", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.serialize_pb()
                .write_to_vec(&mut serialize_buffer)
                .unwrap();
            black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message.write_to_vec(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize_pb()
        .write_to_vec(&mut deserialize_buffer)
        .unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::Message::parse_from_bytes(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "protobuf", deserialize_buffer.as_slice());

    assert!(
        T::Message::parse_from_bytes(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.finish();
}
