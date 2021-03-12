use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::{
    bench_abomonation,
    bench_bincode,
    bench_capnp,
    bench_cbor,
    bench_flatbuffers,
    bench_postcard,
    bench_prost,
    bench_rkyv,
    bench_rmp,
    bench_serde_json,
    datasets::log::{Address, Log, Logs},
    generate_vec,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    const BENCH: &'static str = "log";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const LOGS: usize = 10_000;
    let data = Logs {
        logs: generate_vec::<_, Log>(&mut rng, LOGS..LOGS + 1),
    };

    bench_abomonation::bench(BENCH, c, &data, |data| {
        for log in data.logs.iter() {
            black_box(log.address);
            black_box(log.code);
            black_box(log.size);
        }
    });

    bench_bincode::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader = capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader.get_root::<rust_serialization_benchmark::datasets::log::cp::logs::Reader>().unwrap();
        for log in data.get_logs().unwrap().iter() {
            black_box(log.get_address().unwrap());
            black_box(log.get_code());
            black_box(log.get_size());
        }
    });

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| {
        let data = flatbuffers::get_root::<rust_serialization_benchmark::datasets::log::fb::Logs>(bytes);
        for log in data.logs().iter() {
            black_box(log.address());
            black_box(log.code());
            black_box(log.size_());
        }
    });

    bench_postcard::bench(BENCH, c, &data);

    bench_prost::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data, 
        |data| {
            for log in data.logs.iter() {
                black_box(log.address);
                black_box(log.code);
                black_box(log.size);
            }
        },
        |mut logs| {
            for i in 0..logs.as_ref().logs.len() {
                let mut log = logs.as_mut().logs_pin().index_pin(i);
                *log.as_mut().address_pin() = Address { x0: 0, x1: 0, x2: 0, x3: 0 };
                *log.as_mut().code_pin() = 200;
                *log.as_mut().size_pin() = 0;
            }
        }
    );

    bench_rmp::bench(BENCH, c, &data);

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
