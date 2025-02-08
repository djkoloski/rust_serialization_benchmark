use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    const BUFFER_LEN: usize = 15_000_000;

    let mut group = c.benchmark_group(format!("{}/ciborium", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            ciborium::ser::into_writer(
                black_box(&data),
                black_box(serialize_buffer.as_mut_slice()),
            )
            .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    ciborium::ser::into_writer(&data, &mut deserialize_buffer).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                ciborium::de::from_reader::<T, _>(black_box(deserialize_buffer.as_slice()))
                    .unwrap(),
            );
        })
    });

    crate::bench_size(name, "ciborium", deserialize_buffer.as_slice());

    assert!(ciborium::de::from_reader::<T, _>(deserialize_buffer.as_slice()).unwrap() == *data);

    group.finish();
}

// ciborium only provides reading implementations that require DeserializeOwned, and thus won't
// decode borrowed data at this time.
