use criterion::{criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::{
    bench_abomonation,
    bench_bincode,
    bench_capnp,
    bench_cbor,
    bench_flatbuffers,
    bench_postcard,
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

    bench_abomonation::bench(BENCH, c, &data);

    bench_bincode::bench(BENCH, c, &data);

    bench_capnp::bench(BENCH, c, &data);

    bench_cbor::bench(BENCH, c, &data);

    bench_flatbuffers::bench(BENCH, c, &data);

    bench_postcard::bench(BENCH, c, &data);

    bench_rkyv::bench(BENCH, c, &data, |mut players| {
        for i in 0..players.as_ref().players.len() {
            let mut player = players.as_mut().players_pin().index_pin(i);
            *player.as_mut().game_type_pin() = GameType::Survival;
            *player.as_mut().spawn_x_pin() = 0;
            *player.as_mut().spawn_y_pin() = 0;
            *player.as_mut().spawn_z_pin() = 0;
        }
    });

    bench_serde_json::bench(BENCH, c, &data);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
