use columnar::{
    bytemuck,
    bytes::{EncodeDecode, Indexed},
    Borrow, Clear, Columnar, FromBytes, Index, Push,
};
use criterion::{black_box, Criterion};

pub fn bench<T: Columnar + PartialEq>(name: &'static str, c: &mut Criterion, data: &T) {
    let mut group = c.benchmark_group(format!("{}/columnar", name));

    let mut buffer = Vec::default();
    let mut columns: <T as Columnar>::Container = Default::default();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            columns.clear();
            columns.push(data);
            buffer.clear();
            Indexed::encode(&mut buffer, &columns.borrow());
            black_box(());
        })
    });

    columns.clear();
    columns.push(data);

    // Idiomatic columnar serialization, where your data are SoA (columns) rather than AoS (rows).
    // The difference from "serialize" above is only the data transposition from rows to columns.
    group.bench_function("serialize (SoA)", |b| {
        b.iter(|| {
            buffer.clear();
            Indexed::encode(&mut buffer, &columns.borrow());
            black_box(());
        })
    });

    buffer.clear();
    Indexed::encode(&mut buffer, &columns.borrow());

    group.bench_function("access", |b| {
        b.iter(|| {
            let decoded =
                <<<T as Columnar>::Container as Borrow>::Borrowed<'_> as FromBytes>::from_bytes(
                    &mut Indexed::decode(&buffer),
                );
            black_box(decoded);
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let decoded =
                <<<T as Columnar>::Container as Borrow>::Borrowed<'_> as FromBytes>::from_bytes(
                    &mut Indexed::decode(&buffer),
                );
            black_box(<T as Columnar>::into_owned(decoded.get(0)));
        })
    });

    let decoded = <<<T as Columnar>::Container as Borrow>::Borrowed<'_> as FromBytes>::from_bytes(
        &mut Indexed::decode(&buffer),
    );
    let mut deser: T = <T as Columnar>::into_owned(decoded.get(0));

    // Deserialize into an existing instance, to avoid benchmarking the allocator.
    group.bench_function("deserialize (copy_from)", |b| {
        b.iter(|| {
            let decoded =
                <<<T as Columnar>::Container as Borrow>::Borrowed<'_> as FromBytes>::from_bytes(
                    &mut Indexed::decode(&buffer),
                );
            deser.copy_from(decoded.get(0));
            black_box(&deser);
        })
    });
    crate::bench_size(name, "columnar", bytemuck::cast_slice(&buffer));

    let decoded = <<<T as Columnar>::Container as Borrow>::Borrowed<'_> as FromBytes>::from_bytes(
        &mut Indexed::decode(&buffer),
    );
    let deser: T = <T as Columnar>::into_owned(decoded.get(0));
    assert!(data == &deser);

    group.finish();
}
