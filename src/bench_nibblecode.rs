use criterion::{black_box, Criterion};
use nibblecode::{
    access, access_unchecked, access_unchecked_mut, aligned_alloc::new_uninit_boxed_slice,
    to_bytes, to_bytes_in_unchecked, Serialize,
};

pub fn bench<T, R, U>(name: &'static str, c: &mut Criterion, data: &T, read: R, update: U)
where
    T: Serialize<Archived: PartialEq<T>> + PartialEq,
    R: Fn(&T::Archived),
    U: Fn(&mut T::Archived),
{
    let mut group = c.benchmark_group(format!("{}/nibblecode", name));

    let mut buffer =
        new_uninit_boxed_slice::<T>(data.serialized_size(size_of::<T::Archived>()).unwrap());

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(unsafe { to_bytes_in_unchecked(black_box(data), black_box(&mut buffer)) })
        })
    });

    let buffer = to_bytes(data).unwrap();

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| black_box(unsafe { access_unchecked::<T>(black_box(&*buffer)) }))
    });

    group.bench_function("access (validated upfront with error)", |b| {
        b.iter(|| black_box(access::<T>(black_box(&buffer)).unwrap()))
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            let value = unsafe { access_unchecked::<T>(black_box(&*buffer)) };
            read(value);
        })
    });

    group.bench_function("read (validated upfront with error)", |b| {
        b.iter(|| {
            read(access::<T>(black_box(&*buffer)).unwrap());
        })
    });

    let mut update_buffer = buffer.clone();
    group.bench_function("update (unvalidated)", |b| {
        b.iter(|| {
            let value = unsafe { access_unchecked_mut::<T>(black_box(&mut *update_buffer)) };
            update(value);
        })
    });

    crate::bench_size(name, "nibblecode", &buffer);

    assert!(access::<T>(buffer.as_ref()).unwrap() == data);

    group.finish();
}
