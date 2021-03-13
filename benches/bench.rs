use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::{
    bench_abomonation,
    bench_bincode,
    bench_borsh,
    bench_capnp,
    bench_cbor,
    bench_flatbuffers,
    bench_nachricht,
    bench_postcard,
    bench_prost,
    bench_rkyv,
    bench_rmp,
    bench_serde_json,
    generate_vec,
};

fn bench_log(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::log::{Address, Log, Logs};

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

    bench_borsh::bench(BENCH, c, &data);

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

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| { unsafe {
        let data = flatbuffers::root_unchecked::<rust_serialization_benchmark::datasets::log::fb::Logs>(bytes);
        for log in data.logs().iter() {
            black_box(log.address());
            black_box(log.code());
            black_box(log.size_());
        }
    }});

    bench_nachricht::bench(BENCH, c, &data);

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

fn bench_mesh(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::mesh::{Mesh, Triangle};

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

    bench_borsh::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader = capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader.get_root::<rust_serialization_benchmark::datasets::mesh::cp::mesh::Reader>().unwrap();
        for triangle in data.get_triangles().unwrap().iter() {
            black_box(triangle.get_normal().unwrap());
        }
    });

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| { unsafe {
        let data = flatbuffers::root_unchecked::<rust_serialization_benchmark::datasets::mesh::fb::Mesh>(bytes);
        for triangle in data.triangles().iter() {
            black_box(triangle.normal());
        }
    }});

    bench_nachricht::bench(BENCH, c, &data);

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

fn bench_minecraft_savedata(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::minecraft_savedata::{Player, Players, GameType};

    const BENCH: &'static str = "minecraft_savedata";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const PLAYERS: usize = 500;
    let data = Players {
        players: generate_vec::<_, Player>(&mut rng, PLAYERS..PLAYERS + 1),
    };

    bench_abomonation::bench(BENCH, c, &data, |data| {
        for player in data.players.iter() {
            black_box(player.game_type);
        }
    });

    bench_bincode::bench(BENCH, c, &data);

    bench_borsh::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader = capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader.get_root::<rust_serialization_benchmark::datasets::minecraft_savedata::cp::players::Reader>().unwrap();
        for player in data.get_players().unwrap().iter() {
            black_box(player.get_game_type().unwrap());
        }
    });

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| { unsafe {
        let data = flatbuffers::root_unchecked::<rust_serialization_benchmark::datasets::minecraft_savedata::fb::Players>(bytes);
        for player in data.players().iter() {
            black_box(player.game_type());
        }
    }});

    bench_nachricht::bench(BENCH, c, &data);

    bench_postcard::bench(BENCH, c, &data);

    bench_prost::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data,
        |data| {
            for player in data.players.iter() {
                black_box(player.game_type);
            }
        },
        |mut players| {
            for i in 0..players.as_ref().players.len() {
                let mut player = players.as_mut().players_pin().index_pin(i);
                *player.as_mut().game_type_pin() = GameType::Survival;
                *player.as_mut().spawn_x_pin() = 0;
                *player.as_mut().spawn_y_pin() = 0;
                *player.as_mut().spawn_z_pin() = 0;
            }
        }
    );

    bench_rmp::bench(BENCH, c, &data);

    bench_serde_json::bench(BENCH, c, &data);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_log(c);
    bench_mesh(c);
    bench_minecraft_savedata(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
