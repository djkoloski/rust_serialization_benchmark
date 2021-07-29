use criterion::{black_box, Criterion};
use alkahest::{
    Pack, Schema, Unpacked,
};

pub fn bench<T, D>(name: &'static str, c: &mut Criterion, data: D, read: fn(Unpacked<'_, T>))
where
    T: Schema,
    D: Pack<T> + Copy,
{
    let mut group = c.benchmark_group(format!("{}/alkahest", name));

    // This leak is required because of ICE in rustc if `R` has bound like this `for<'a> Fn(Unpacked<'a, T>)`.
    let mut buffer = vec![0u64; 1 << 1 << 22];
    let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut buffer[..]);
    let mut size = 0;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                size = alkahest::write::<T, _>(bytes, black_box(data))
            );
        })
    });

    group.bench_function("access (validated)", |b| {
        b.iter(|| {
            black_box(
                alkahest::read::<T>(black_box(&bytes[..size]))
            );
        })
    });

    group.bench_function("read (validated)", |b| {
        b.iter(|| {
            black_box(
                read(alkahest::read::<T>(black_box(&bytes[..size])))
            );
        })
    });

    println!("{}/alkahest/size {}", name, size);
    println!("{}/alkahest/zlib {}", name, crate::zlib_size(&bytes[..size]));

    group.finish();
}
