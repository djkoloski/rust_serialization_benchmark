use alkahest::{Pack, Schema, Unpacked};
use criterion::{black_box, Criterion};

pub fn bench<T, D>(name: &'static str, c: &mut Criterion, data: D, read: fn(Unpacked<'_, T>))
where
    T: Schema,
    D: Pack<T> + Copy,
{
    let mut group = c.benchmark_group(format!("{}/alkahest", name));

    // This leak is required because of ICE in rustc if `R` has bound like this `for<'a> Fn(Unpacked<'a, T>)`.
    const BUFFER_LEN: usize = 10_000_000;
    let mut buffer = vec![0; BUFFER_LEN];

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(alkahest::write::<T, _>(&mut buffer, black_box(data)));
        })
    });

    let size = alkahest::write::<T, _>(&mut buffer, data);
    let buffer = &buffer[..size];

    group.bench_function("access (validated on-demand with panic)", |b| {
        b.iter(|| {
            black_box(alkahest::read::<T>(black_box(buffer)));
        })
    });

    group.bench_function("read (validated on-demand with panic)", |b| {
        b.iter(|| {
            read(alkahest::read::<T>(black_box(buffer)));
            black_box(());
        })
    });

    crate::bench_size(name, "alkahest", buffer);

    group.finish();
}
