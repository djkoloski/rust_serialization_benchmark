use borsh::{BorshSerialize, BorshDeserialize};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BorshSerialize + BorshDeserialize,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/borsh", name));

    let mut serialize_buffer = vec![0u8; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                data.serialize(black_box(&mut serialize_buffer.as_mut_slice()))
                    .unwrap()
            );
        })
    });

    let deserialize_buffer = data.try_to_vec().unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                T::deserialize(&mut deserialize_buffer.as_slice()).unwrap()
            );
        })
    });

    crate::bench_size(name, "borsh", deserialize_buffer.as_slice());

    group.finish();
}
