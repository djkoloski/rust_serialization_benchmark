use criterion::{black_box, Criterion};
use speedy::{Endianness, Readable, Writable};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: for<'a> Readable<'a, Endianness> + Writable<Endianness>,
{
    const BUFFER_LEN: usize = 10_000_000;

    #[cfg(target_endian = "little")]
    const CONTEXT: Endianness = Endianness::LittleEndian;
    #[cfg(target_endian = "big")]
    const CONTEXT: Endianness = Endianness::BigEndian;

    let mut group = c.benchmark_group(format!("{}/speedy", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                data.write_to_buffer_with_ctx(CONTEXT, black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    let deserialize_buffer = data.write_to_vec_with_ctx(CONTEXT).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                T::read_from_buffer_with_ctx(CONTEXT, &black_box(deserialize_buffer.as_slice()))
                    .unwrap(),
            );
        })
    });

    crate::bench_size(name, "speedy", deserialize_buffer.as_slice());

    group.finish();
}
