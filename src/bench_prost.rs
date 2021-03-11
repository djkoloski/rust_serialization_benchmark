use criterion::{black_box, Criterion};
use prost::Message;

pub trait Serialize {
    type Message: Default + Message;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/prost", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            black_box(data.serialize_pb().encode(&mut serialize_buffer).unwrap());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize_pb().encode(&mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::Message::decode(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    println!("prost size: {} bytes", deserialize_buffer.len());
    println!("prost zlib size: {} bytes", crate::zlib_size(deserialize_buffer.as_slice()));

    group.finish();
}
