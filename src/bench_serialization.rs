use std::fmt::Debug;

use criterion::{black_box, Criterion};
use fastbuf::{Buf, Buffer, ReadBuf};
use serialization::{Decode, Encode};
use serialization_minecraft::{PacketDecoder, PacketEncoder};

pub fn bench<'de, T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Encode + Decode + PartialEq + Debug,
{
    let mut group = c.benchmark_group(format!("{}/serialization", name));

    const BUFFER_LEN: usize = 200_000;
    let mut buf = unsafe { Box::<Buffer<BUFFER_LEN>>::new_zeroed().assume_init() };

    group.bench_function("serialize", |b| {
        b.iter(|| {
            unsafe { buf.set_filled_pos(0) };
            let ref mut encoder = PacketEncoder::new(&mut buf);
            let _result = black_box(&black_box(data).encode(encoder).unwrap());
        })
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let ref mut decoder = PacketDecoder::new(&mut buf);
            let _result = black_box(&T::decode_placed(decoder).unwrap());
            unsafe { buf.set_pos(0) };
        })
    });
    crate::bench_size(name, "serialization", buf.get_continuous(buf.remaining()));

    let ref mut decoder = PacketDecoder::new(&mut buf);
    assert!(data == &T::decode_placed(decoder).unwrap());

    group.finish();
}
