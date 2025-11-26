use core::slice;
use std::mem::MaybeUninit;

use criterion::{black_box, Criterion};
use wincode::{SchemaRead, SchemaWrite};

use crate::datasets::BorrowableData;

pub trait SerializeSelf: SchemaWrite<Src = Self> {}
impl<T> SerializeSelf for T where T: SchemaWrite<Src = T> {}

pub trait DeserializeSelf<'de>: SchemaRead<'de, Dst = Self> {}
impl<'de, T> DeserializeSelf<'de> for T where T: SchemaRead<'de, Dst = T> {}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: SerializeSelf + for<'de> DeserializeSelf<'de> + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/wincode", name));

    let mut buffer = vec![0u8; wincode::serialized_size(data).unwrap() as usize];
    let ptr = buffer.as_mut_ptr().cast::<MaybeUninit<u8>>();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            let mut target = unsafe { slice::from_raw_parts_mut(ptr, buffer.len()) };
            wincode::serialize_into(black_box(&mut target), black_box(&data)).unwrap();
        })
    });

    let buffer = wincode::serialize(data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(wincode::deserialize::<T>(black_box(&buffer)).unwrap());
        })
    });

    crate::bench_size(name, "wincode", &buffer);

    assert!(wincode::deserialize::<T>(black_box(&buffer)).unwrap() == *data);

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: SerializeSelf + for<'de> DeserializeSelf<'de> + PartialEq + BorrowableData,
    for<'a> T::Borrowed<'a>: SerializeSelf + DeserializeSelf<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/wincode", name));

    {
        // The borrowed variant type should encode exactly the same as the owned type.
        let serialized = wincode::serialize(&data).unwrap();
        let deserialized = wincode::deserialize::<T::Borrowed<'_>>(&serialized).unwrap();
        let bdata = T::Borrowed::from(data);
        // // The borrowed value we decode should be equivalent to the input
        assert!(bdata == deserialized);
    }

    let serialized = wincode::serialize(&data).unwrap();
    let buffer = &serialized[..];

    group.bench_function("borrow", |b| {
        b.iter(|| {
            let _data =
                black_box(wincode::deserialize::<T::Borrowed<'_>>(black_box(buffer)).unwrap());
        })
    });

    group.finish();
}
