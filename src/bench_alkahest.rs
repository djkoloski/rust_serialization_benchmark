use alkahest::{Pack, Schema, Unpacked};
use criterion::{black_box, Criterion};

pub fn bench<T, D>(name: &'static str, c: &mut Criterion, data: D, read: fn(Unpacked<'_, T>))
where
    T: Schema,
    D: Pack<T> + Copy,
{
    let mut group = c.benchmark_group(format!("{}/alkahest", name));

    // This leak is required because of ICE in rustc if `R` has bound like this `for<'a> Fn(Unpacked<'a, T>)`.
    const BUFFER_LEN: usize = 8_388_608;
    let mut buffer = vec![0u64; BUFFER_LEN];
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer[..]);
    let mut size = 0;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            size = alkahest::write::<T, _>(bytes, black_box(data));
            black_box(());
        })
    });

    group.bench_function("access (validated on-demand with panic)", |b| {
        b.iter(|| {
            black_box(alkahest::read::<T>(black_box(&bytes[..size])));
        })
    });

    group.bench_function("read (validated on-demand with panic)", |b| {
        b.iter(|| {
            read(alkahest::read::<T>(black_box(&bytes[..size])));
            black_box(());
        })
    });

    crate::bench_size(name, "alkahest", &bytes[..size]);

    group.finish();
}
