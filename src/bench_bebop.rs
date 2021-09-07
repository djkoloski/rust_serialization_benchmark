use bebop::Record;
use criterion::{black_box, Criterion};

pub trait Serialize<'a> {
    type Target: 'a + Record<'a>;

    fn populate_bb(&'a self) -> Self::Target;
    fn depopulate_bb(target: Self::Target) -> Self;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    for<'a> T: Serialize<'a>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/bebop", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + serialize)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            black_box(data.populate_bb().serialize(&mut serialize_buffer).unwrap());
        })
    });

    let populated = data.populate_bb();
    group.bench_function("serialize (serialize)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            black_box(populated.serialize(&mut serialize_buffer).unwrap());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.populate_bb().serialize(&mut deserialize_buffer).unwrap();

    group.bench_function("access", |b| {
        b.iter(|| {
            black_box(T::Target::deserialize(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::depopulate_bb(&T::Target::deserialize(black_box(&deserialize_buffer).as_slice()).unwrap()));
        })
    });

    crate::bench_size(name, "bebop", deserialize_buffer.as_slice());

    group.finish();
}
