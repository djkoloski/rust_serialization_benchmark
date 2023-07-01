use rand::Rng;
#[cfg(feature = "rkyv")]
use rkyv::Archived;

use crate::{generate_vec, Generate};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "alkahest", derive(alkahest::Schema))]
#[repr(u8)]
pub enum EntityType {
    ArleighBurke,
    Bismarck,
    Clemenceau,
    Fletcher,
    G5,
    Iowa,
    Kolkata,
    Osa,
    Yasen,
    Zubr,
}

impl EntityType {
    fn turret_count(self) -> usize {
        use EntityType::*;
        match self {
            ArleighBurke => 1,
            Bismarck => 4,
            Clemenceau => 4,
            Fletcher => 4,
            G5 => 0,
            Iowa => 3,
            Kolkata => 1,
            Osa => 2,
            Yasen => 0,
            Zubr => 3,
        }
    }

    fn weapon_count(self) -> usize {
        use EntityType::*;
        match self {
            ArleighBurke => 20,
            Bismarck => 10,
            Clemenceau => 16,
            Fletcher => 18,
            G5 => 2,
            Iowa => 28,
            Kolkata => 15,
            Osa => 6,
            Yasen => 14,
            Zubr => 11,
        }
    }
}

impl Generate for EntityType {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        // TODO put in lazy_static or similar.
        // TODO actual weights.
        let index = rand::distributions::WeightedIndex::new([1.0; 10]).unwrap();

        use EntityType::*;
        match rng.sample(index) {
            0 => ArleighBurke,
            1 => Bismarck,
            2 => Clemenceau,
            3 => Fletcher,
            4 => G5,
            5 => Iowa,
            6 => Kolkata,
            7 => Osa,
            8 => Yasen,
            9 => Zubr,
            _ => unreachable!(),
        }
    }
}

fn generate_altitude(rng: &mut impl Rng) -> i8 {
    if rng.gen_bool(0.1) {
        rng.gen_range(-120..-35)
    } else {
        0
    }
}

fn generate_velocity(rng: &mut impl Rng) -> i16 {
    const MIN_SPEED: i16 = 140;
    const MAX_SPEED: i16 = 900;
    const REVERSE_SLOW: i16 = 4;

    match rng.gen_range(0..100) {
        0..=79 => rng.gen_range(MIN_SPEED..MAX_SPEED),
        80..=89 => 0,
        90..=100 => rng.gen_range((-MAX_SPEED / REVERSE_SLOW)..(-MIN_SPEED / REVERSE_SLOW)),
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Transform {
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..1"))]
    pub altitude: i8,
    pub angle: u16,
    pub position: (f32, f32),
    pub velocity: i16,
}

impl Generate for Transform {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const RANGE: f32 = 750.0;
        Self {
            altitude: generate_altitude(rng),
            angle: rng.gen(),
            position: (rng.gen_range(-RANGE..RANGE), rng.gen_range(-RANGE..RANGE)),
            velocity: generate_velocity(rng),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Guidance {
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..1"))]
    pub altitude: i8,
    pub angle: u16,
    pub velocity: i16,
}

impl Generate for Guidance {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            altitude: generate_altitude(rng),
            angle: rng.gen(),
            velocity: generate_velocity(rng),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "alkahest", derive(alkahest::Schema))]
pub struct Contact {
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..1"))]
    pub damage: u8,
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..10000"))]
    pub entity_id: u32,
    pub entity_type: Option<EntityType>,
    pub guidance: Guidance,
    pub player_id: u32,
    pub reloads: Vec<bool>,
    pub transform: Transform,
    pub turret_angles: Vec<u16>,
}

impl Generate for Contact {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        let entity_type = rng.gen_bool(0.7).then(|| Generate::generate(rng));
        let entity_count = rng.gen_range(4000..10000);

        Self {
            damage: if rng.gen_bool(0.6) {
                0
            } else {
                rng.gen_range(0..200)
            },
            entity_id: rng.gen_range(0..entity_count),
            entity_type,
            guidance: Generate::generate(rng),
            player_id: rng.gen(),
            reloads: entity_type
                .map(|entity_type| {
                    let p = rng.gen_range(0.0..1.0);
                    (0..entity_type.weapon_count())
                        .map(|_| rng.gen_bool(p))
                        .collect()
                })
                .unwrap_or_default(),
            transform: Generate::generate(rng),
            turret_angles: entity_type
                .map(|entity_type| {
                    let base_angle: u16 = rng.gen();
                    (0..entity_type.turret_count())
                        .map(|_| {
                            if rng.gen_bool(0.75) {
                                (base_angle as i16 + rng.gen_range(-200..200) as i16) as u16
                            } else {
                                rng.gen()
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct TerrainUpdate {
    x: i8,
    y: i8,
    data: Vec<u8>,
}

impl Generate for TerrainUpdate {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        let radius = rng.gen_range(5..7);
        let is_new = rng.gen_bool(0.1);

        Self {
            x: rng.gen_range(-radius..radius),
            y: rng.gen_range(-radius..radius),
            data: generate_vec(rng, if is_new { 1000..2000 } else { 20..50 }),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Update {
    pub contacts: Vec<Contact>,
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..5000"))]
    pub score: u32,
    pub world_radius: f32,
    pub terrain_updates: Vec<TerrainUpdate>,
}

impl Generate for Update {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            contacts: (0..rng.gen_range(40..80))
                .map(|_| Generate::generate(rng))
                .collect(),
            score: (rng.gen::<f32>().powi(4) * 5000.0) as u32 + rng.gen_range(0..50),
            world_radius: rng.gen_range(1500.0..2500.0),
            terrain_updates: (0..rng.gen_range(0..5))
                .map(|_| Generate::generate(rng))
                .collect(),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
pub struct Updates {
    pub updates: Vec<Update>,
}

#[cfg(feature = "rkyv")]
const _: () = {
    use core::pin::Pin;

    impl ArchivedUpdates {
        pub fn updates_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Update>>> {
            unsafe { self.map_unchecked_mut(|s| &mut s.updates) }
        }
    }
};
