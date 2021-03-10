use criterion::{criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::{
    bench_abomonation,
    bench_bincode,
    bench_capnp,
    bench_flatbuffers,
    bench_postcard,
    bench_rkyv,
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

    bench_abomonation::bench(BENCH, c, &data);

    bench_bincode::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data);

    bench_postcard::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data, |mut logs| {
        for i in 0..logs.as_ref().logs.len() {
            let mut log = logs.as_mut().logs_pin().index_pin(i);
            *log.as_mut().address_pin() = Address { x0: 0, x1: 0, x2: 0, x3: 0 };
            *log.as_mut().code_pin() = 200;
            *log.as_mut().size_pin() = 0;
        }
    });

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
