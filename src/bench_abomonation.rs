use abomonation::{decode, encode, Abomonation};
use criterion::{black_box, Criterion};

pub fn bench<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: Abomonation + Clone,
    R: Fn(&T),
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/abomonation", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            unsafe {
                encode(data, black_box(&mut serialize_buffer)).unwrap();
                black_box(());
            }
        })
    });

    let mut deserialize_buffer = Vec::new();
    unsafe {
        encode(data, &mut deserialize_buffer).unwrap();
    }

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| unsafe {
            black_box(decode::<T>(black_box(&mut deserialize_buffer)).unwrap());
        })
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| unsafe {
            let (data, _) = decode::<T>(black_box(&mut deserialize_buffer)).unwrap();
            read(data);
            black_box(());
        })
    });

    group.bench_function("deserialize (unvalidated)", |b| {
        b.iter(|| {
            let (data, _) = unsafe { decode::<T>(black_box(&mut deserialize_buffer)).unwrap() };
            black_box((*data).clone());
        })
    });

    crate::bench_size(name, "abomonation", deserialize_buffer.as_slice());

    group.finish();
}
