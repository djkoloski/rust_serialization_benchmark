use criterion::{black_box, Criterion};
use flatbuffers::{FlatBufferBuilder, Follow, Verifiable, WIPOffset};

pub trait Serialize<'a> {
    type Target: 'a + Follow<'a> + Verifiable;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T, read_unverified: impl Fn(&[u8]), read_verified: impl Fn(&[u8]))
where
    T: for<'a> Serialize<'a>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/flatbuffers", name));

    let mut fbb = FlatBufferBuilder::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut fbb).reset();
            let root = data.serialize_fb(&mut fbb);
            fbb.finish(root, None);
            black_box(&mut fbb);
        })
    });

    fbb.reset();
    let root = data.serialize_fb(&mut fbb);
    fbb.finish(root, None);
    let deserialize_buffer = fbb.finished_data();

    group.bench_function("access (unvalidated)", |b| {
        b.iter(|| { unsafe {
            black_box(flatbuffers::root_unchecked::<<T as Serialize<'_>>::Target>(black_box(deserialize_buffer)))
        }})
    });

    group.bench_function("access (validated upfront with error)", |b| {
        b.iter(|| {
            black_box(flatbuffers::root::<<T as Serialize<'_>>::Target>(black_box(deserialize_buffer)).unwrap())
        })
    });

    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| {
            black_box(read_unverified(&black_box(deserialize_buffer)))
        })
    });

    group.bench_function("read (validated upfront with error)", |b| {
        b.iter(|| {
            black_box(read_verified(&black_box(deserialize_buffer)))
        })
    });

    println!("{}/flatbuffers/size {}", name, deserialize_buffer.len());
    println!("{}/flatbuffers/zlib {}", name, crate::zlib_size(deserialize_buffer));

    group.finish();
}
