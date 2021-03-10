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
    datasets::mesh::{Mesh, Triangle},
    generate_vec,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    const BENCH: &'static str = "mesh";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const TRIANGLES: usize = 125_000;
    let data = Mesh {
        triangles: generate_vec::<_, Triangle>(&mut rng, TRIANGLES..TRIANGLES + 1),
    };

    bench_abomonation::bench(BENCH, c, &data);

    bench_bincode::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data);

    bench_postcard::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data, |mut mesh| {
        for i in 0..mesh.as_ref().triangles.len() {
            let mut triangle = mesh.as_mut().triangles_pin().index_pin(i);
            triangle.normal.x = 0f32;
            triangle.normal.y = 0f32;
            triangle.normal.z = 0f32;
        }
    });

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
