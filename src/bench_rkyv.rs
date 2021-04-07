use bytecheck::CheckBytes;
use core::pin::Pin;
use criterion::{black_box, Criterion};
use rkyv::{
    archived_value,
    archived_value_mut,
    check_archived_value,
    de::deserializers::AllocDeserializer,
    ser::{Serializer, serializers::AlignedSerializer},
    validation::{DefaultArchiveValidator},
    AlignedVec,
    Archive,
    Deserialize,
    Serialize,
};

pub fn bench<T, R, U>(name: &'static str, c: &mut Criterion, data: &T, read: R, update: U)
where
    T: Archive + for<'a> Serialize<AlignedSerializer<&'a mut AlignedVec>>,
    T::Archived: CheckBytes<DefaultArchiveValidator> + Deserialize<T, AllocDeserializer>,
    R: Fn(&T::Archived),
    U: Fn(Pin<&mut T::Archived>),
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/rkyv", name));

    let mut serialize_buffer = AlignedVec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            let mut serializer = AlignedSerializer::new(black_box(&mut serialize_buffer));
            black_box(
                serializer.serialize_value(black_box(data)).unwrap()
            );
        })
    });

    serialize_buffer.clear();
    let mut serializer = AlignedSerializer::new(&mut serialize_buffer);
    let pos = serializer.serialize_value(data).unwrap();
    let deserialize_buffer = serializer.into_inner();

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| {
            black_box(unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            });
        })
    });

    group.bench_function("access (validated)", |b| {
        b.iter(|| {
            black_box(
                check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos)).unwrap()
            );
        })
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            black_box(unsafe {
                read(archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos)))
            });
        })
    });

    group.bench_function("read (validated)", |b| {
        b.iter(|| {
            black_box(
                read(check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos)).unwrap())
            );
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
    
    println!("{}/rkyv/size {}", name, deserialize_buffer.len());
    println!("{}/rkyv/zlib {}", name, crate::zlib_size(deserialize_buffer.as_slice()));

    group.finish();
}