use core::pin::Pin;
use criterion::{black_box, Criterion};
use rkyv::{
    archived_value,
    archived_value_mut,
    de::deserializers::AllocDeserializer,
    ser::{Serializer, serializers::WriteSerializer},
    Archive,
    Deserialize,
    Serialize,
};

pub fn bench<T, F>(name: &'static str, c: &mut Criterion, data: &T, update: F)
where
    T: Archive + Serialize<WriteSerializer<Vec<u8>>> + for<'a> Serialize<WriteSerializer<&'a mut [u8]>>,
    T::Archived: Deserialize<T, AllocDeserializer>,
    F: Fn(Pin<&mut T::Archived>),
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/rkyv", name));

    let mut serialize_buffer = vec![0u8; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let mut serializer = WriteSerializer::new(black_box(serialize_buffer.as_mut_slice()));
            black_box(
                serializer.serialize_value(black_box(data)).unwrap()
            );
        })
    });

    let mut serializer = WriteSerializer::new(Vec::new());
    let pos = serializer.serialize_value(data).unwrap();
    let mut deserialize_buffer = serializer.into_inner();

    group.bench_function("access", |b| {
        b.iter(|| {
            black_box(unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            });
        })
    });

    group.bench_function("update", |b| {
        b.iter(|| {
            let mut value = unsafe {
                archived_value_mut::<T>(black_box(Pin::new_unchecked(deserialize_buffer.as_mut_slice())), black_box(pos))
            };
            update(value.as_mut());
            black_box(value);
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let value = unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            };
            let deserialized: T = value.deserialize(&mut AllocDeserializer).unwrap();
            black_box(deserialized);
        })
    });
    
    println!("rkyv size: {} bytes", deserialize_buffer.len());

    group.finish();
}