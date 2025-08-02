use crate::datasets::BorrowableData;
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

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BorrowableData + Serialize + for<'de> Deserialize<'de>,
    for<'a> T::Borrowed<'a>: Serialize + Deserialize<'a>,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/flexbuffers", name));

    let mut builder = flexbuffers::FlexbufferSerializer::new();
    data.serialize(&mut builder).unwrap();
    let bdata = T::Borrowed::from(data);

    // The borrowed variant type should encode exactly the same as the owned type.
    let mut borrowed_builder = flexbuffers::FlexbufferSerializer::new();
    bdata.serialize(&mut borrowed_builder).unwrap();
    assert_eq!(builder.view(), borrowed_builder.view());

    let deserialize_buffer = builder.view();

    // The borrowed value we decode should be equivalent to the input
    assert!(
        T::Borrowed::<'_>::deserialize(flexbuffers::Reader::get_root(deserialize_buffer).unwrap())
            .unwrap()
            == bdata
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                T::Borrowed::<'_>::deserialize(black_box(
                    flexbuffers::Reader::get_root(black_box(deserialize_buffer)).unwrap(),
                ))
                .unwrap(),
            );
        });
    });

    group.finish();
}
