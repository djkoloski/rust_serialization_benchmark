#[allow(unused_imports)]
use criterion::{black_box, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
#[cfg(feature = "bilrost")]
use rust_serialization_benchmark::bench_bilrost;
#[cfg(feature = "bincode")]
use rust_serialization_benchmark::bench_bincode;
#[cfg(feature = "bincode1")]
use rust_serialization_benchmark::bench_bincode1;
#[cfg(feature = "bitcode")]
use rust_serialization_benchmark::bench_bitcode;
#[cfg(feature = "borsh")]
use rust_serialization_benchmark::bench_borsh;
#[cfg(feature = "capnp")]
use rust_serialization_benchmark::bench_capnp;
#[cfg(feature = "cbor4ii")]
use rust_serialization_benchmark::bench_cbor4ii;
#[cfg(feature = "ciborium")]
use rust_serialization_benchmark::bench_ciborium;
#[cfg(feature = "databuf")]
use rust_serialization_benchmark::bench_databuf;
#[cfg(feature = "dlhn")]
use rust_serialization_benchmark::bench_dlhn;
#[cfg(feature = "flatbuffers")]
use rust_serialization_benchmark::bench_flatbuffers;
#[cfg(feature = "msgpacker")]
use rust_serialization_benchmark::bench_msgpacker;
#[cfg(feature = "nachricht-serde")]
use rust_serialization_benchmark::bench_nachricht_serde;
#[cfg(feature = "nanoserde")]
use rust_serialization_benchmark::bench_nanoserde;
#[cfg(feature = "scale")]
use rust_serialization_benchmark::bench_parity_scale_codec;
#[cfg(feature = "postcard")]
use rust_serialization_benchmark::bench_postcard;
#[cfg(feature = "pot")]
use rust_serialization_benchmark::bench_pot;
#[cfg(feature = "prost")]
use rust_serialization_benchmark::bench_prost;
#[cfg(feature = "rkyv")]
use rust_serialization_benchmark::bench_rkyv;
#[cfg(feature = "rmp-serde")]
use rust_serialization_benchmark::bench_rmp_serde;
#[cfg(feature = "ron")]
use rust_serialization_benchmark::bench_ron;
#[cfg(feature = "savefile")]
use rust_serialization_benchmark::bench_savefile;
#[cfg(feature = "serde_bare")]
use rust_serialization_benchmark::bench_serde_bare;
#[cfg(feature = "serde-brief")]
use rust_serialization_benchmark::bench_serde_brief;
#[cfg(feature = "serde_cbor")]
use rust_serialization_benchmark::bench_serde_cbor;
#[cfg(feature = "serde_json")]
use rust_serialization_benchmark::bench_serde_json;
#[cfg(feature = "serialization")]
use rust_serialization_benchmark::bench_serialization;
#[cfg(feature = "simd-json")]
use rust_serialization_benchmark::bench_simd_json;
#[cfg(feature = "speedy")]
use rust_serialization_benchmark::bench_speedy;
#[cfg(feature = "wiring")]
use rust_serialization_benchmark::bench_wiring;

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

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "bilrost")]
    bench_bilrost::bench(BENCH, c, &data);

    #[cfg(feature = "bincode1")]
    bench_bincode1::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde-brief")]
    bench_serde_brief::bench(BENCH, c, &data);

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

    #[cfg(feature = "cbor4ii")]
    bench_cbor4ii::bench(BENCH, c, &data);

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "databuf")]
    bench_databuf::bench(BENCH, c, &data);

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);

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

    #[cfg(feature = "msgpacker")]
    bench_msgpacker::bench(BENCH, c, &data);

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "pot")]
    bench_pot::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |logs| {
            for log in logs.logs.iter() {
                black_box(&log.address);
                black_box(log.code);
                black_box(log.size);
            }
        },
        |log| {
            use rkyv::{munge::munge, vec::ArchivedVec};
            use rust_serialization_benchmark::datasets::log::{
                ArchivedAddress, ArchivedLog, ArchivedLogs,
            };

            munge!(let ArchivedLogs { logs } = log);
            let mut logs = ArchivedVec::as_slice_seal(logs);
            for i in 0..logs.len() {
                munge! {
                    let ArchivedLog {
                        address: ArchivedAddress {
                            mut x0,
                            mut x1,
                            mut x2,
                            mut x3,
                        },
                        mut code,
                        mut size,
                        ..
                    } = logs.as_mut().index(i);
                }
                *x0 = 0;
                *x1 = 0;
                *x2 = 0;
                *x3 = 0;
                *code = 200.into();
                *size = 0.into();
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "savefile")]
    bench_savefile::bench(BENCH, c, &data);

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

    #[cfg(feature = "nanoserde")]
    bench_nanoserde::bench(BENCH, c, &data);

    #[cfg(feature = "wiring")]
    bench_wiring::bench(BENCH, c, &data);
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

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "bilrost")]
    bench_bilrost::bench(BENCH, c, &data);

    #[cfg(feature = "bincode1")]
    bench_bincode1::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde-brief")]
    bench_serde_brief::bench(BENCH, c, &data);

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

    #[cfg(feature = "cbor4ii")]
    bench_cbor4ii::bench(BENCH, c, &data);

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "databuf")]
    bench_databuf::bench(BENCH, c, &data);

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);

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

    #[cfg(feature = "msgpacker")]
    bench_msgpacker::bench(BENCH, c, &data);

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "pot")]
    bench_pot::bench(BENCH, c, &data);

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
        |mesh| {
            use rkyv::{munge::munge, vec::ArchivedVec};
            use rust_serialization_benchmark::datasets::mesh::{
                ArchivedMesh, ArchivedTriangle, ArchivedVector3,
            };

            munge!(let ArchivedMesh { triangles } = mesh);
            let mut triangles = ArchivedVec::as_slice_seal(triangles);

            for i in 0..triangles.len() {
                munge! {
                    let ArchivedTriangle {
                        normal: ArchivedVector3 {
                            mut x,
                            mut y,
                            mut z,
                        },
                        ..
                    } = triangles.as_mut().index(i);
                }
                *x = 0f32.into();
                *y = 0f32.into();
                *z = 0f32.into();
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "savefile")]
    bench_savefile::bench(BENCH, c, &data);

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

    #[cfg(feature = "nanoserde")]
    bench_nanoserde::bench(BENCH, c, &data);

    #[cfg(feature = "wiring")]
    bench_wiring::bench(BENCH, c, &data);
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

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "bilrost")]
    bench_bilrost::bench(BENCH, c, &data);

    #[cfg(feature = "bincode1")]
    bench_bincode1::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde-brief")]
    bench_serde_brief::bench(BENCH, c, &data);

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

    #[cfg(feature = "cbor4ii")]
    bench_cbor4ii::bench(BENCH, c, &data);

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "databuf")]
    bench_databuf::bench(BENCH, c, &data);

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);

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

    #[cfg(feature = "msgpacker")]
    bench_msgpacker::bench(BENCH, c, &data);

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "pot")]
    bench_pot::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |players| {
            for player in players.players.iter() {
                black_box(&player.game_type);
            }
        },
        |players| {
            use rkyv::{munge::munge, vec::ArchivedVec};
            use rust_serialization_benchmark::datasets::minecraft_savedata::{
                ArchivedPlayer, ArchivedPlayers,
            };

            use rust_serialization_benchmark::datasets::minecraft_savedata::ArchivedGameType;

            munge!(let ArchivedPlayers { players } = players);
            let mut players = ArchivedVec::as_slice_seal(players);

            for i in 0..players.len() {
                munge! {
                    let ArchivedPlayer {
                        mut game_type,
                        mut spawn_x,
                        mut spawn_y,
                        mut spawn_z,
                        ..
                    } = players.as_mut().index(i);
                }
                *game_type = ArchivedGameType::Survival;
                *spawn_x = 0.into();
                *spawn_y = 0.into();
                *spawn_z = 0.into();
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "savefile")]
    bench_savefile::bench(BENCH, c, &data);

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

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "nanoserde")]
    bench_nanoserde::bench(BENCH, c, &data);

    #[cfg(feature = "wiring")]
    bench_wiring::bench(BENCH, c, &data);
}

fn bench_mk48(c: &mut Criterion) {
    use rust_serialization_benchmark::datasets::mk48::Updates;

    const BENCH: &'static str = "mk48";

    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const UPDATES: usize = 1000;
    let data = Updates {
        updates: generate_vec(&mut rng, UPDATES..UPDATES + 1),
    };

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "bilrost")]
    bench_bilrost::bench(BENCH, c, &data);

    #[cfg(feature = "bincode1")]
    bench_bincode1::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "bitcode")]
    bench_bitcode::bench(BENCH, c, &data);

    #[cfg(feature = "borsh")]
    bench_borsh::bench(BENCH, c, &data);

    #[cfg(feature = "serde-brief")]
    bench_serde_brief::bench(BENCH, c, &data);

    #[cfg(feature = "capnp")]
    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader =
            capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader
            .get_root::<rust_serialization_benchmark::datasets::mk48::cp::updates::Reader>()
            .unwrap();
        for update in data.get_updates().unwrap().iter() {
            black_box(update.get_score());
        }
    });

    #[cfg(feature = "cbor4ii")]
    bench_cbor4ii::bench(BENCH, c, &data);

    #[cfg(feature = "ciborium")]
    bench_ciborium::bench(BENCH, c, &data);

    #[cfg(feature = "databuf")]
    bench_databuf::bench(BENCH, c, &data);

    #[cfg(feature = "dlhn")]
    bench_dlhn::bench(BENCH, c, &data);

    #[cfg(feature = "flatbuffers")]
    bench_flatbuffers::bench(
        BENCH,
        c,
        &data,
        |bytes| unsafe {
            let data = flatbuffers::root_unchecked::<
                rust_serialization_benchmark::datasets::mk48::fb::Updates,
            >(bytes);
            for update in data.updates().iter() {
                black_box(update.score());
            }
        },
        |bytes| {
            let data =
                flatbuffers::root::<rust_serialization_benchmark::datasets::mk48::fb::Updates>(
                    bytes,
                )
                .unwrap();
            for update in data.updates().iter() {
                black_box(update.score());
            }
        },
    );

    #[cfg(feature = "msgpacker")]
    bench_msgpacker::bench(BENCH, c, &data);

    #[cfg(feature = "nachricht-serde")]
    bench_nachricht_serde::bench(BENCH, c, &data);

    #[cfg(feature = "scale")]
    bench_parity_scale_codec::bench(BENCH, c, &data);

    #[cfg(feature = "postcard")]
    bench_postcard::bench(BENCH, c, &data);

    #[cfg(feature = "pot")]
    bench_pot::bench(BENCH, c, &data);

    #[cfg(feature = "prost")]
    bench_prost::bench(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(
        BENCH,
        c,
        &data,
        |updates| {
            for update in updates.updates.iter() {
                black_box(update.score);
            }
        },
        |updates| {
            use rkyv::{munge::munge, vec::ArchivedVec};
            use rust_serialization_benchmark::datasets::mk48::{ArchivedUpdate, ArchivedUpdates};

            munge!(let ArchivedUpdates { updates } = updates);
            let mut updates = ArchivedVec::as_slice_seal(updates);

            for i in 0..updates.len() {
                munge!(let ArchivedUpdate { mut score, .. } = updates.as_mut().index(i));
                *score *= 2;
            }
        },
    );

    #[cfg(feature = "rmp-serde")]
    bench_rmp_serde::bench(BENCH, c, &data);

    #[cfg(feature = "ron")]
    bench_ron::bench(BENCH, c, &data);

    #[cfg(feature = "savefile")]
    bench_savefile::bench(BENCH, c, &data);

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

    #[cfg(feature = "serialization")]
    bench_serialization::bench(BENCH, c, &data);

    #[cfg(feature = "nanoserde")]
    bench_nanoserde::bench(BENCH, c, &data);

    #[cfg(feature = "wiring")]
    bench_wiring::bench(BENCH, c, &data);
}

#[cfg(feature = "pprof")]
mod profiling {
    use criterion::profiler::Profiler;
    use pprof::ProfilerGuard;
    use std::ffi::c_int;
    use std::fs::File;
    use std::path::Path;

    pub struct FlamegraphProfiler<'a> {
        frequency: c_int,
        active_profiler: Option<ProfilerGuard<'a>>,
    }

    impl<'a> FlamegraphProfiler<'a> {
        pub fn new(frequency: c_int) -> Self {
            FlamegraphProfiler {
                frequency,
                active_profiler: None,
            }
        }
    }

    impl<'a> Profiler for FlamegraphProfiler<'a> {
        fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
            self.active_profiler = Some(ProfilerGuard::new(self.frequency).unwrap());
        }

        fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
            std::fs::create_dir_all(benchmark_dir).unwrap();
            let flamegraph_path = benchmark_dir.join("flamegraph.svg");
            let flamegraph_file = File::create(&flamegraph_path)
                .expect("File system error while creating flamegraph.svg");
            if let Some(profiler) = self.active_profiler.take() {
                profiler
                    .report()
                    .build()
                    .unwrap()
                    .flamegraph(flamegraph_file)
                    .expect("Error writing flamegraph");
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_log(c);
    bench_mesh(c);
    bench_minecraft_savedata(c);
    bench_mk48(c);
}

pub fn benches() {
    let criterion = Criterion::default();
    #[cfg(feature = "pprof")]
    let criterion = criterion.with_profiler(profiling::FlamegraphProfiler::new(100));
    let mut criterion = criterion.configure_from_args();
    criterion_benchmark(&mut criterion);
}

criterion_main!(benches);
