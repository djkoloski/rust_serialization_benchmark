use compactly::{Encode};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/compactly", name));

        group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(compactly::encode(black_box(data)));
        })
    });

    let encoded = compactly::encode(data).to_vec();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(compactly::decode::<T>(black_box(&encoded)).unwrap());
        })
    });

    crate::bench_size(name, "compactly", &encoded);

    assert!(compactly::decode::<T>(&encoded).unwrap() == *data);

    group.finish();
}
