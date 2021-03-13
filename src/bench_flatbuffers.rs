use criterion::{black_box, Criterion};
use flatbuffers::{FlatBufferBuilder, Follow, WIPOffset};

pub trait Serialize<'a> {
    type Target: 'a + Follow<'a>;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b;
}

pub fn bench<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: for<'a> Serialize<'a>,
    R: Fn(&[u8]),
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/flatbuffers", name));

    let mut fbb = FlatBufferBuilder::new_with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut fbb).reset();
            let root = data.serialize_fb(&mut fbb);
            fbb.finish(root, None);
            black_box(&mut fbb);
        })
    });

    black_box(&mut fbb).reset();
    let root = data.serialize_fb(&mut fbb);
    fbb.finish(root, None);
    let deserialize_buffer = fbb.finished_data();

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| { unsafe {
            black_box(flatbuffers::root_unchecked::<<T as Serialize<'_>>::Target>(black_box(deserialize_buffer)))
        }})
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            black_box(read(&black_box(deserialize_buffer)))
        })
    });

    println!("{}/flatbuffers/size {}", name, deserialize_buffer.len());
    println!("{}/flatbuffers/zlib {}", name, crate::zlib_size(deserialize_buffer));

    group.finish();
}
