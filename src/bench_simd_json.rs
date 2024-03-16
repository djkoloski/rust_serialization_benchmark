use criterion::{black_box, BatchSize, Criterion};
use simd_json::Buffers;
use simd_json_derive::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/simd-json", name));

    let mut serialize_buffer = vec![0u8; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(data)
                .json_write(&mut black_box(serialize_buffer.as_mut_slice()))
                .unwrap();
            black_box(());
        })
    });

    let deserialize_buffer = data.json_vec().unwrap();
    let mut buffers = Buffers::new(BUFFER_LEN);

    group.bench_function("deserialize", |b| {
        b.iter_batched_ref(
            || deserialize_buffer.clone(),
            |deserialize_buffer| {
                black_box(
                    T::from_slice_with_buffers(deserialize_buffer.as_mut_slice(), &mut buffers)
                        .unwrap(),
                );
            },
            BatchSize::SmallInput,
        )
    });

    crate::bench_size(name, "simd-json", deserialize_buffer.as_slice());

    assert!(
        T::from_slice_with_buffers(deserialize_buffer.clone().as_mut_slice(), &mut buffers)
            .unwrap()
            == *data
    );

    group.finish();
}
