use criterion::{criterion_group, criterion_main, Criterion};
use rand_pcg::Lcg64Xsh32;
use rust_serialization_benchmark::{
    bench_bincode,
    bench_flatbuffers,
    bench_rkyv,
    datasets::minecraft_savedata::{Player, Players, GameType},
    generate_vec,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    // nothing up our sleeves, state and stream are first 20 digits of pi
    const STATE: u64 = 3141592653;
    const STREAM: u64 = 5897932384;

    let mut rng = Lcg64Xsh32::new(STATE, STREAM);

    const PLAYERS: usize = 500;
    let data = Players {
        players: generate_vec::<_, Player>(&mut rng, PLAYERS..PLAYERS + 1),
    };

    bench_bincode::bench(c, &data, |players| {
        for player in players.players.iter_mut() {
            player.game_type = GameType::Survival;
            player.spawn_x = 0;
            player.spawn_y = 0;
            player.spawn_z = 0;
        }
    });

    bench_flatbuffers::bench(c, &data);

    bench_rkyv::bench(c, &data, |mut players| {
        for i in 0..players.as_ref().players.len() {
            let mut player = players.as_mut().players_pin().index_pin(i);
            *player.as_mut().game_type_pin() = GameType::Survival;
            *player.as_mut().spawn_x_pin() = 0;
            *player.as_mut().spawn_y_pin() = 0;
            *player.as_mut().spawn_z_pin() = 0;
        }
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
