use core::mem::take;

use criterion::{black_box, Criterion};
use rkyv::{
    access, access_unchecked, access_unchecked_mut,
    api::high::{to_bytes_in, HighDeserializer, HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    deserialize, from_bytes,
    rancor::Failure,
    seal::Seal,
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    Archive, Deserialize, Serialize,
};

pub type BenchSerializer<'a> = HighSerializer<'a, AlignedVec, ArenaHandle<'a>, Failure>;
pub type BenchDeserializer = HighDeserializer<Failure>;
pub type BenchValidator<'a> = HighValidator<'a, Failure>;

pub fn bench<T, R, U>(name: &'static str, c: &mut Criterion, data: &T, read: R, update: U)
where
    T: Archive + for<'a> Serialize<BenchSerializer<'a>> + PartialEq,
    T::Archived: for<'a> CheckBytes<BenchValidator<'a>> + Deserialize<T, BenchDeserializer>,
    R: Fn(&T::Archived),
    U: for<'a> Fn(Seal<'a, T::Archived>),
    <T as Archive>::Archived: for<'a> CheckBytes<BenchValidator<'a>>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/rkyv", name));

    let mut buffer = AlignedVec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize", |b| {
        b.iter(|| {
            buffer.clear();
            buffer = black_box(to_bytes_in(black_box(data), black_box(take(&mut buffer))).unwrap());
        })
    });

    buffer.clear();
    buffer = black_box(to_bytes_in(black_box(data), black_box(buffer)).unwrap());

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| black_box(unsafe { access_unchecked::<T::Archived>(black_box(buffer.as_ref())) }))
    });

    group.bench_function("access (validated upfront with error)", |b| {
        b.iter(|| black_box(access::<T::Archived, Failure>(black_box(buffer.as_ref())).unwrap()))
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            black_box(unsafe { read(access_unchecked::<T::Archived>(black_box(buffer.as_ref()))) })
        })
    });

    group.bench_function("read (validated upfront with error)", |b| {
        b.iter(|| {
            black_box(read(
                access::<T::Archived, Failure>(black_box(buffer.as_ref())).unwrap(),
            ))
        })
    });

    let mut update_buffer = buffer.clone();
    group.bench_function("update (unvalidated)", |b| {
        b.iter(|| {
            let mut value = unsafe {
                access_unchecked_mut::<T::Archived>(black_box(update_buffer.as_mut_slice()))
            };
            update(value.as_mut());
            black_box(value);
        })
    });

    group.bench_function("deserialize (unvalidated)", |b| {
        b.iter(|| {
            let value = unsafe { access_unchecked::<T::Archived>(black_box(buffer.as_ref())) };
            black_box(deserialize(value).unwrap())
        })
    });

    group.bench_function("deserialize (validated upfront with error)", |b| {
        b.iter(|| {
            let value = access::<T::Archived, Failure>(black_box(buffer.as_ref())).unwrap();
            black_box(deserialize(value).unwrap())
        })
    });

    crate::bench_size(name, "rkyv", &buffer);

    assert!(from_bytes::<T, Failure>(buffer.as_ref()).unwrap() == *data);

    group.finish();
}
