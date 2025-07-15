use criterion::{black_box, Criterion};
use serde::{Deserialize, Serialize};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/flexbuffers", name));

    let mut builder = flexbuffers::FlexbufferSerializer::new();

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut builder).reset();
            data.serialize(&mut builder).unwrap();
            black_box(());
        })
    });

    builder.reset();
    data.serialize(&mut builder).unwrap();

    let deserialize_buffer = builder.view();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                T::deserialize(black_box(
                    flexbuffers::Reader::get_root(black_box(deserialize_buffer)).unwrap(),
                ))
                .unwrap(),
            );
        })
    });

    crate::bench_size(name, "flexbuffers", deserialize_buffer);

    group.finish();
}
