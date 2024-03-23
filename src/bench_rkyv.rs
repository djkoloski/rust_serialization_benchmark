use core::pin::Pin;

use bytecheck::CheckBytes;
use criterion::{black_box, Criterion};
use rkyv::{
    archived_value, archived_value_mut, check_archived_value,
    ser::{
        serializers::{AlignedSerializer, BufferScratch, CompositeSerializer},
        Serializer,
    },
    validation::validators::DefaultValidator,
    AlignedVec, Archive, Deserialize, Infallible, Serialize,
};

pub type BenchSerializer<'a> = CompositeSerializer<
    AlignedSerializer<&'a mut AlignedVec>,
    BufferScratch<&'a mut AlignedVec>,
    Infallible,
>;

pub fn bench<T, R, U>(name: &'static str, c: &mut Criterion, data: &T, read: R, update: U)
where
    T: Archive + for<'a> Serialize<BenchSerializer<'a>> + PartialEq,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, Infallible>,
    R: Fn(&T::Archived),
    U: Fn(Pin<&mut T::Archived>),
    <T as Archive>::Archived: for<'a> rkyv::CheckBytes<DefaultValidator<'a>>,
{
    const BUFFER_LEN: usize = 10_000_000;
    const SCRATCH_LEN: usize = 512_000;

    let mut group = c.benchmark_group(format!("{}/rkyv", name));

    let mut serialize_buffer = AlignedVec::with_capacity(BUFFER_LEN);
    let mut serialize_scratch = AlignedVec::with_capacity(SCRATCH_LEN);
    unsafe {
        serialize_scratch.set_len(SCRATCH_LEN);
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();

            let mut serializer = CompositeSerializer::new(
                AlignedSerializer::new(black_box(&mut serialize_buffer)),
                BufferScratch::new(black_box(&mut serialize_scratch)),
                Infallible,
            );
            black_box(serializer.serialize_value(black_box(data)).unwrap());
        })
    });

    serialize_buffer.clear();
    let mut serializer = CompositeSerializer::new(
        AlignedSerializer::new(&mut serialize_buffer),
        BufferScratch::new(&mut serialize_scratch),
        Infallible,
    );
    let pos = serializer.serialize_value(data).unwrap();
    let deserialize_buffer = serializer.into_serializer().into_inner();

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| {
            black_box(unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            });
        })
    });

    group.bench_function("access (validated upfront with error)", |b| {
        b.iter(|| {
            black_box(
                check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
                    .unwrap(),
            );
        })
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            unsafe {
                read(archived_value::<T>(
                    black_box(deserialize_buffer.as_ref()),
                    black_box(pos),
                ))
            };
            black_box(());
        })
    });

    group.bench_function("read (validated upfront with error)", |b| {
        b.iter(|| {
            read(
                check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
                    .unwrap(),
            );
            black_box(());
        })
    });

    let mut update_buffer = deserialize_buffer.clone();
    group.bench_function("update", |b| {
        b.iter(|| {
            let mut value = unsafe {
                archived_value_mut::<T>(
                    black_box(Pin::new_unchecked(update_buffer.as_mut_slice())),
                    black_box(pos),
                )
            };
            update(value.as_mut());
            black_box(value);
        })
    });

    group.bench_function("deserialize (unvalidated)", |b| {
        b.iter(|| {
            let value = unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            };
            let deserialized: T = value.deserialize(&mut Infallible).unwrap();
            black_box(deserialized);
        })
    });

    group.bench_function("deserialize (validated upfront with error)", |b| {
        b.iter(|| {
            let value =
                check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
                    .unwrap();
            let deserialized: T = value.deserialize(&mut Infallible).unwrap();
            black_box(deserialized);
        })
    });

    crate::bench_size(name, "rkyv", deserialize_buffer);

    assert!(
        check_archived_value::<T>(deserialize_buffer.as_ref(), pos)
            .unwrap()
            .deserialize(&mut Infallible)
            .unwrap()
            == *data
    );

    group.finish();
}
