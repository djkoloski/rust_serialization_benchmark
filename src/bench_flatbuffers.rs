use criterion::{black_box, Criterion};
use flatbuffers::{FlatBufferBuilder, Follow, WIPOffset};

pub trait Serialize<'a> {
    type Target: 'a + Follow<'a>;

    fn serialize_fb<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b;
}

pub fn bench<T>(c: &mut Criterion, data: &T)
where
    T: for<'a> Serialize<'a>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group("flatbuffers");

    let mut builder = FlatBufferBuilder::new_with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut builder).reset();
            let root = data.serialize_fb(&mut builder);
            builder.finish(root, None);
            black_box(&mut builder);
        })
    });

    black_box(&mut builder).reset();
    let root = data.serialize_fb(&mut builder);
    builder.finish(root, None);
    let deserialize_buffer = builder.finished_data();

    group.bench_function("access", |b| {
        b.iter(|| {
            black_box(flatbuffers::get_root::<<T as Serialize<'_>>::Target>(black_box(deserialize_buffer)))
        })
    });

    println!("flatbuffers size: {} bytes", deserialize_buffer.len());

    group.finish();
}
