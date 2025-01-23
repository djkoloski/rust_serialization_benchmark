use criterion::{black_box, Criterion};
use minicbor::{Decode, Encode};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode<()> + for<'de> Decode<'de, ()> + PartialEq,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/minicbor", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            minicbor::encode(black_box(data), black_box(serialize_buffer.as_mut_slice())).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    minicbor::encode(&data, &mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(minicbor::decode::<'_, T>(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "minicbor", deserialize_buffer.as_slice());

    assert!(minicbor::decode::<T>(&deserialize_buffer).unwrap() == *data);

    group.finish();
}
