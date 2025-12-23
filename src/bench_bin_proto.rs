use bin_proto::{BitCodec, LittleEndian};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: BitCodec + PartialEq,
{
    const ENDIANNESS: LittleEndian = LittleEndian;

    let mut group = c.benchmark_group(format!("{}/bin-proto", name));

    let encoded_data = data.encode_bytes(ENDIANNESS).unwrap();

    let mut serialize_buffer = vec![0; encoded_data.len()];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                data.encode_bytes_buf(ENDIANNESS, black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                T::decode_bytes(black_box(&encoded_data), ENDIANNESS)
                    .unwrap()
                    .0,
            );
        })
    });

    crate::bench_size(name, "bin-proto", &encoded_data);

    assert!(T::decode_bytes(&encoded_data, ENDIANNESS).unwrap().0 == *data);

    group.finish();
}

// bin-proto does not support borrowed decoding.
