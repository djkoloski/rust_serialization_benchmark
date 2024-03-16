use core::fmt;
use criterion::{black_box, BatchSize, Criterion};
use msgpacker::prelude::*;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Packable + Unpackable + PartialEq,
    <T as Unpackable>::Error: fmt::Debug,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/msgpacker", name));

    group.bench_function("serialize", |b| {
        b.iter_batched(
            || Vec::with_capacity(BUFFER_LEN),
            |mut buf| T::pack(black_box(data), &mut buf),
            BatchSize::SmallInput,
        );
    });

    let mut deserialize_buffer = Vec::new();
    T::pack(data, &mut deserialize_buffer);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            T::unpack(black_box(&deserialize_buffer)).unwrap();
        })
    });

    crate::bench_size(name, "msgpacker", &deserialize_buffer);

    assert!(T::unpack(&deserialize_buffer).unwrap().1 == *data);

    group.finish();
}
