use criterion::{black_box, Criterion};
use nanoserde::{DeBin, SerBin};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: DeBin + SerBin + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/nanoserde", name));

    group.bench_function("serialize", |b| {
        b.iter(|| {
            SerBin::serialize_bin(black_box(data));
            black_box(());
        })
    });

    let deserialize_buffer = SerBin::serialize_bin(data);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(<T as DeBin>::deserialize_bin(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "nanoserde", deserialize_buffer.as_slice());

    assert!(<T as DeBin>::deserialize_bin(&deserialize_buffer).unwrap() == *data);

    group.finish();
}

// nanoserde does not appear to support borrowed decoding.
