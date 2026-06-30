use capnp::{message::ScratchSpaceHeapAllocator, serialize::read_message_from_flat_slice};
use criterion::{black_box, Criterion};

pub trait Serialize<'a> {
    type Reader: capnp::traits::FromPointerReader<'a>;
    type Builder: capnp::traits::FromPointerBuilder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder);
}

pub fn bench<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: for<'a> Serialize<'a>,
    R: Fn(&mut &[u8]),
{
    const BUFFER_LEN: usize = 1_000_000;

    let mut group = c.benchmark_group(format!("{}/capnp", name));

    let mut serialize_buffer = Vec::new();

    let mut scratch_words = capnp::Word::allocate_zeroed_vec(BUFFER_LEN);
    let mut allocator =
        ScratchSpaceHeapAllocator::new(capnp::Word::words_to_bytes_mut(&mut scratch_words[..]));
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            let mut builder = capnp::message::Builder::new(&mut allocator);
            data.serialize_capnp(&mut builder.init_root::<T::Builder>());
            capnp::serialize::write_message(&mut serialize_buffer, &builder).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    let mut builder = capnp::message::Builder::new(&mut allocator);
    data.serialize_capnp(&mut builder.init_root::<T::Builder>());
    capnp::serialize::write_message(&mut deserialize_buffer, &builder).unwrap();

    group.bench_function("access (validated on-demand with error)", |b| {
        b.iter(|| {
            black_box(&mut deserialize_buffer);
            let message_reader = read_message_from_flat_slice(
                &mut deserialize_buffer.as_slice(),
                Default::default(),
            )
            .unwrap();
            let reader = message_reader.get_root::<T::Reader>().unwrap();
            black_box(reader);
        })
    });

    group.bench_function("read (validated on-demand with error)", |b| {
        b.iter(|| {
            read(black_box(&mut deserialize_buffer.as_slice()));
            black_box(());
        })
    });

    crate::bench_size(name, "capnp", deserialize_buffer.as_slice());

    group.finish();
}

// Packed format: same wire schema, zero-byte runs stripped. Requires an unpack
// step on decode, so it's not zero-copy. It's reported as `(packed)` variants of
// the `capnp` benchmark — `serialize`/`deserialize` and size in the ser/de table
// — rather than as a separate library row.
pub fn bench_packed<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: for<'a> Serialize<'a>,
    R: Fn(&mut &[u8]),
{
    const BUFFER_LEN: usize = 1_000_000;

    let mut group = c.benchmark_group(format!("{}/capnp", name));

    let mut serialize_buffer = Vec::new();

    let mut scratch_words = capnp::Word::allocate_zeroed_vec(BUFFER_LEN);
    let mut allocator =
        ScratchSpaceHeapAllocator::new(capnp::Word::words_to_bytes_mut(&mut scratch_words[..]));
    group.bench_function("serialize (packed)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            let mut builder = capnp::message::Builder::new(&mut allocator);
            data.serialize_capnp(&mut builder.init_root::<T::Builder>());
            capnp::serialize_packed::write_message(&mut serialize_buffer, &builder).unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    let mut builder = capnp::message::Builder::new(&mut allocator);
    data.serialize_capnp(&mut builder.init_root::<T::Builder>());
    capnp::serialize_packed::write_message(&mut deserialize_buffer, &builder).unwrap();

    group.bench_function("read (packed)", |b| {
        b.iter(|| {
            read(black_box(&mut deserialize_buffer.as_slice()));
            black_box(());
        })
    });

    crate::bench_size_variant(name, "capnp", "packed", deserialize_buffer.as_slice());

    group.finish();
}

// capnp is a pseudo-zerocopy library with incremental decoding and does not support borrowed
// decoding.
