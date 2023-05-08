use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
#[cfg(feature = "abomonation")]
use rust_serialization_benchmark::bench_abomonation;
#[cfg(feature = "alkahest")]
use rust_serialization_benchmark::bench_alkahest;
#[cfg(feature = "bincode")]
use rust_serialization_benchmark::bench_bincode;
#[cfg(feature = "bitcode")]
use rust_serialization_benchmark::bench_bitcode;
#[cfg(feature = "borsh")]
use rust_serialization_benchmark::bench_borsh;
#[cfg(feature = "bson")]
use rust_serialization_benchmark::bench_bson;
#[cfg(feature = "capnp")]
use rust_serialization_benchmark::bench_capnp;
#[cfg(feature = "ciborium")]
use rust_serialization_benchmark::bench_ciborium;
#[cfg(feature = "dlhn")]
use rust_serialization_benchmark::bench_dlhn;
#[cfg(feature = "flatbuffers")]
use rust_serialization_benchmark::bench_flatbuffers;
#[cfg(feature = "nachricht-serde")]
use rust_serialization_benchmark::bench_nachricht_serde;
#[cfg(feature = "postcard")]
use rust_serialization_benchmark::bench_postcard;
#[cfg(feature = "prost")]
use rust_serialization_benchmark::bench_prost;
#[cfg(feature = "rkyv")]
use rust_serialization_benchmark::bench_rkyv;
#[cfg(feature = "rmp-serde")]
use rust_serialization_benchmark::bench_rmp_serde;
#[cfg(feature = "ron")]
use rust_serialization_benchmark::bench_ron;
#[cfg(feature = "scale")]
use rust_serialization_benchmark::bench_parity_scale_codec;
#[cfg(feature = "serde_bare")]
use rust_serialization_benchmark::bench_serde_bare;
#[cfg(feature = "serde_cbor")]
use rust_serialization_benchmark::bench_serde_cbor;
#[cfg(feature = "serde_json")]
use rust_serialization_benchmark::bench_serde_json;
#[cfg(feature = "simd-json")]
use rust_serialization_benchmark::bench_simd_json;
#[cfg(feature = "speedy")]
use rust_serialization_benchmark::bench_speedy;
use rust_serialization_benchmark::generate_vec;

fn bench_log(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::log::{Log, Logs};

    const BENCH: &'static str = "log";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const LOGS: usize = 10_000;
    let data = Logs {
        logs: generate_vec::<_, Log>(&mut rng, LOGS..LOGS + 1),
    };

    #[cfg(feature = "abomonation")]
    bench_abomonation::bench(BENCH, c, &data, |data| {
        for log in data.logs.iter() {
            black_box(log.address);
            black_box(log.code);
            black_box(log.size);
        }
    });

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "bson")]
    bench_bson::bench(BENCH, c, &data);

    #[cfg(feature = "capnp")]
    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader =
            capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader
            .get_root::<rust_serialization_benchmark::datasets::log::cp::logs::Reader>()
            .unwrap();
        for log in data.get_logs().unwrap().iter() {
            black_box(log.get_address().unwrap());
            black_box(log.get_code());
            black_box(log.get_size());
        }
    });

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "flatbuffers")]
    bench_flatbuffers::bench(
        BENCH,
        c,
        &data,
        |bytes| unsafe {
            let data = flatbuffers::root_unchecked::<
                rust_serialization_benchmark::datasets::log::fb::Logs,
            >(bytes);
            for log in data.logs().iter() {
                black_box(log.address());
                black_box(log.code());
                black_box(log.size_());
            }
        },
        |bytes| {
            let data =
                flatbuffers::root::<rust_serialization_benchmark::datasets::log::fb::Logs>(bytes)
                    .unwrap();
            for log in data.logs().iter() {
                black_box(log.address());
                black_box(log.code());
                black_box(log.size_());
            }
        },
    );

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |data| {
            for log in data.logs.iter() {
                black_box(&log.address);
                black_box(log.code);
                black_box(log.size);
            }
        },
        |mut logs| {
            use rust_serialization_benchmark::datasets::log::ArchivedAddress;

            for i in 0..logs.as_ref().logs.len() {
                let mut log = logs.as_mut().logs_pin().index_pin(i);
                *log.as_mut().address_pin() = ArchivedAddress {
                    x0: 0,
                    x1: 0,
                    x2: 0,
                    x3: 0,
                };
                *log.as_mut().code_pin() = 200;
                *log.as_mut().size_pin() = 0;
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "serde_bare")]
    bench_serde_bare::bench(BENCH, c, &data);

    #[cfg(feature = "serde_cbor")]
    bench_serde_cbor::bench(BENCH, c, &data);

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "simd-json")]
    bench_simd_json::bench(BENCH, c, &data);

    #[cfg(feature = "speedy")]
    bench_speedy::bench(BENCH, c, &data);

    // Doesn't use a closure due to ICE in rustc. Probably related to https://github.com/rust-lang/rust/issues/86703
    #[cfg(feature = "alkahest")]
    bench_alkahest::bench(BENCH, c, &data, |data| {
        for log in data.logs {
            black_box(&log.address);
            black_box(log.code);
            black_box(log.size);
        }
    });

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);
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

    #[cfg(feature = "abomonation")]
    bench_abomonation::bench(BENCH, c, &data, |data| {
        for triangle in data.triangles.iter() {
            black_box(triangle.normal);
        }
    });

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "bson")]
    bench_bson::bench(BENCH, c, &data);

    #[cfg(feature = "capnp")]
    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader =
            capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader
            .get_root::<rust_serialization_benchmark::datasets::mesh::cp::mesh::Reader>()
            .unwrap();
        for triangle in data.get_triangles().unwrap().iter() {
            black_box(triangle.get_normal().unwrap());
        }
    });

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "flatbuffers")]
    bench_flatbuffers::bench(
        BENCH,
        c,
        &data,
        |bytes| unsafe {
            let data = flatbuffers::root_unchecked::<
                rust_serialization_benchmark::datasets::mesh::fb::Mesh,
            >(bytes);
            for triangle in data.triangles().iter() {
                black_box(triangle.normal());
            }
        },
        |bytes| {
            let data =
                flatbuffers::root::<rust_serialization_benchmark::datasets::mesh::fb::Mesh>(bytes)
                    .unwrap();
            for triangle in data.triangles().iter() {
                black_box(triangle.normal());
            }
        },
    );

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |mesh| {
            for triangle in mesh.triangles.iter() {
                black_box(&triangle.normal);
            }
        },
        |mut mesh| {
            for i in 0..mesh.as_ref().triangles.len() {
                let mut triangle = mesh.as_mut().triangles_pin().index_pin(i);
                triangle.normal.x = 0f32;
                triangle.normal.y = 0f32;
                triangle.normal.z = 0f32;
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "serde_bare")]
    bench_serde_bare::bench(BENCH, c, &data);

    #[cfg(feature = "serde_cbor")]
    bench_serde_cbor::bench(BENCH, c, &data);

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "simd-json")]
    bench_simd_json::bench(BENCH, c, &data);

    #[cfg(feature = "speedy")]
    bench_speedy::bench(BENCH, c, &data);

    // Doesn't use a closure due to ICE in rustc. Probably related to https://github.com/rust-lang/rust/issues/86703
    #[cfg(feature = "alkahest")]
    bench_alkahest::bench(BENCH, c, &data, |mesh| {
        for triangle in mesh.triangles {
            black_box(&triangle.normal);
        }
    });

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);
}

fn bench_minecraft_savedata(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::minecraft_savedata::{Player, Players};

    const BENCH: &'static str = "minecraft_savedata";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const PLAYERS: usize = 500;
    let data = Players {
        players: generate_vec::<_, Player>(&mut rng, PLAYERS..PLAYERS + 1),
    };

    #[cfg(feature = "abomonation")]
    bench_abomonation::bench(BENCH, c, &data, |data| {
        for player in data.players.iter() {
            black_box(player.game_type);
        }
    });

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "bson")]
    bench_bson::bench(BENCH, c, &data);

    #[cfg(feature = "capnp")]
    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader =
            capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader
      .get_root::<rust_serialization_benchmark::datasets::minecraft_savedata::cp::players::Reader>()
      .unwrap();
        for player in data.get_players().unwrap().iter() {
            black_box(player.get_game_type().unwrap());
        }
    });

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "flatbuffers")]
    bench_flatbuffers::bench(
        BENCH,
        c,
        &data,
        |bytes| unsafe {
            let data = flatbuffers::root_unchecked::<
                rust_serialization_benchmark::datasets::minecraft_savedata::fb::Players,
            >(bytes);
            for player in data.players().iter() {
                black_box(player.game_type());
            }
        },
        |bytes| {
            let data = flatbuffers::root::<
                rust_serialization_benchmark::datasets::minecraft_savedata::fb::Players,
            >(bytes)
            .unwrap();
            for player in data.players().iter() {
                black_box(player.game_type());
            }
        },
    );

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |data| {
            for player in data.players.iter() {
                black_box(&player.game_type);
            }
        },
        |mut players| {
            use rust_serialization_benchmark::datasets::minecraft_savedata::ArchivedGameType;

            for i in 0..players.as_ref().players.len() {
                let mut player = players.as_mut().players_pin().index_pin(i);
                *player.as_mut().game_type_pin() = ArchivedGameType::Survival;
                *player.as_mut().spawn_x_pin() = 0;
                *player.as_mut().spawn_y_pin() = 0;
                *player.as_mut().spawn_z_pin() = 0;
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "serde_bare")]
    bench_serde_bare::bench(BENCH, c, &data);

    #[cfg(feature = "serde_cbor")]
    bench_serde_cbor::bench(BENCH, c, &data);

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "simd-json")]
    bench_simd_json::bench(BENCH, c, &data);

    #[cfg(feature = "speedy")]
    bench_speedy::bench(BENCH, c, &data);

    #[cfg(feature = "alkahest")]
    bench_alkahest::bench(BENCH, c, &data, |data| {
        for player in data.players {
            black_box(&player.game_type);
        }
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_log(c);
    bench_mesh(c);
    bench_minecraft_savedata(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
