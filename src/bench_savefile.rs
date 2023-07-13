use criterion::{black_box, Criterion};
use savefile::{Deserialize, Serialize, WithSchema};
use std::io::Cursor;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + Deserialize + WithSchema,
{
    let mut group = c.benchmark_group(format!("{}/savefile", name));

    let mut serialize_buffer = Vec::new();
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            savefile::save_noschema(black_box(&mut serialize_buffer), 0, black_box(data)).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    savefile::save_noschema(&mut deserialize_buffer, 0, data).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let mut reader = Cursor::new(&deserialize_buffer);
            black_box(savefile::load_noschema::<T>(black_box(&mut reader), 0).unwrap());
        })
    });

    crate::bench_size(name, "savefile", deserialize_buffer.as_slice());

    group.finish();
}
