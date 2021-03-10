use abomonation::{Abomonation, encode, decode};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Abomonation,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/abomonation", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            unsafe {
                black_box(
                    encode(data, black_box(&mut serialize_buffer))
                    .unwrap()
                );
            }
        })
    });

    let mut deserialize_buffer = Vec::new();
    unsafe { encode(data, &mut deserialize_buffer).unwrap(); }

    group.bench_function("access", |b| {
        b.iter(|| {
            unsafe {
                black_box(
                    decode::<T>(black_box(&mut deserialize_buffer)).unwrap()
                );
            }
        })
    });

    println!("abomonation size: {} bytes", deserialize_buffer.len());
}