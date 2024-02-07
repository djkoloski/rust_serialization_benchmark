use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/dlhn", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&data)
                .serialize(&mut dlhn::ser::Serializer::new(&mut black_box(
                    serialize_buffer.as_mut_slice(),
                )))
                .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize(&mut dlhn::ser::Serializer::new(&mut deserialize_buffer))
        .unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                <T>::deserialize(&mut dlhn::de::Deserializer::new(black_box(
                    &mut deserialize_buffer.as_slice(),
                )))
                .unwrap(),
            );
        })
    });

    crate::bench_size(name, "dlhn", deserialize_buffer.as_slice());

    assert!(
        <T>::deserialize(&mut dlhn::de::Deserializer::new(
            &mut deserialize_buffer.as_slice()
        ))
        .unwrap()
            == *data
    );

    group.finish();
}
