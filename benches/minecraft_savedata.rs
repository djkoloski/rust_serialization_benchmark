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
    bench_serde_json,
    datasets::minecraft_savedata::{Player, Players, GameType},
    generate_vec,
};

pub fn criterion_benchmark(c: &mut Criterion) {
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

    bench_capnp::bench(BENCH, c, &data, |bytes| {
        let message_reader = capnp::serialize::read_message_from_flat_slice(bytes, Default::default()).unwrap();
        let data = message_reader.get_root::<rust_serialization_benchmark::datasets::minecraft_savedata::cp::players::Reader>().unwrap();
        for player in data.get_players().unwrap().iter() {
            black_box(player.get_game_type().unwrap());
        }
    });

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data, |bytes| {
        let data = flatbuffers::get_root::<rust_serialization_benchmark::datasets::minecraft_savedata::fb::Players>(bytes);
        for player in data.players().iter() {
            black_box(player.game_type());
        }
    });

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

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
