use bin_proto::{BitDecode, BitDecodeExt, BitEncode, BitEncodeExt, LittleEndian};
use criterion::{black_box, Criterion};

type Endianness = LittleEndian;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BitEncode<Endianness> + BitDecode<Endianness> + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/bin-proto", name));

    let encoded_data = data.encode_bytes().unwrap();

    let mut serialize_buffer = vec![0; encoded_data.len()];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                data.encode_bytes_buf(black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::decode_bytes(black_box(&encoded_data)).unwrap().0);
        })
    });

    crate::bench_size(name, "bin-proto", &encoded_data);

    assert!(T::decode_bytes(&encoded_data).unwrap().0 == *data);

    group.finish();
}

// bin-proto does not support borrowed decoding.
