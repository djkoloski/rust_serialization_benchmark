use criterion::{Criterion, black_box, criterion_group, criterion_main};
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

    bench_abomonation::bench(BENCH, c, &data, |data| {
        for triangle in data.triangles.iter() {
            black_box(triangle.normal);
        }
    });

    bench_bincode::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader = capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader.get_root::<rust_serialization_benchmark::datasets::mesh::cp::mesh::Reader>().unwrap();
        for triangle in data.get_triangles().unwrap().iter() {
            black_box(triangle.get_normal().unwrap());
        }
    });

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| {
        let data = flatbuffers::get_root::<rust_serialization_benchmark::datasets::mesh::fb::Mesh>(bytes);
        for triangle in data.triangles().iter() {
            black_box(triangle.normal());
        }
    });

    bench_postcard::bench(BENCH, c, &data);

    bench_prost::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data, 
        |mesh| {
            for triangle in mesh.triangles.iter() {
                black_box(triangle.normal);
            }
        },
        |mut mesh| {
            for i in 0..mesh.as_ref().triangles.len() {
                let mut triangle = mesh.as_mut().triangles_pin().index_pin(i);
                triangle.normal.x = 0f32;
                triangle.normal.y = 0f32;
                triangle.normal.z = 0f32;
            }
        }
    );

    bench_rmp::bench(BENCH, c, &data);

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
