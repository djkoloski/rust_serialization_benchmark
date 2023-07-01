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
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 2.14))]
    ArleighBurke,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 0.52))]
    Bismarck,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 0.97))]
    Clemenceau,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 1.46))]
    Fletcher,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 13.16))]
    G5,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 1.55))]
    Iowa,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 0.83))]
    Kolkata,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 7.25))]
    Osa,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 4.06))]
    Yasen,
    #[cfg_attr(feature = "bitcode", bitcode_hint(frequency = 12.92))]
    Zubr,
}

impl EntityType {
    fn frequency(self) -> f32 {
        use EntityType::*;
        match self {
            ArleighBurke => 2.14,
            Bismarck => 0.52,
            Clemenceau => 0.97,
            Fletcher => 1.46,
            G5 => 13.16,
            Iowa => 1.55,
            Kolkata => 0.83,
            Osa => 7.25,
            Yasen => 4.06,
            Zubr => 12.92,
        }
    }

    fn is_sub(self) -> bool {
        matches!(self, Self::Yasen)
    }

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
        use EntityType::*;
        const ENTITY_TYPES: [EntityType; 10] = [
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
        ];

        // TODO put in lazy_static or similar.
        let index =
            rand::distributions::WeightedIndex::new(ENTITY_TYPES.map(EntityType::frequency))
                .unwrap();
        ENTITY_TYPES[rng.sample(index)]
    }
}

fn generate_altitude(rng: &mut impl Rng, entity_type: EntityType) -> i8 {
    if entity_type.is_sub() && rng.gen_bool(0.9) {
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

impl Transform {
    fn generate<R: Rng>(rng: &mut R, entity_type: EntityType) -> Self {
        const RANGE: f32 = 750.0;
        Self {
            altitude: generate_altitude(rng, entity_type),
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

impl Guidance {
    fn generate<R: Rng>(rng: &mut R, entity_type: EntityType) -> Self {
        Self {
            altitude: generate_altitude(rng, entity_type),
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
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..8000"))]
    pub entity_id: u32,
    pub entity_type: Option<EntityType>,
    pub guidance: Guidance,
    #[cfg_attr(feature = "bitcode", bitcode_hint(expected_range = "0..400"))]
    pub player_id: Option<u16>,
    pub reloads: Vec<bool>,
    pub transform: Transform,
    pub turret_angles: Vec<u16>,
}

impl Contact {
    fn generate<R: Rng>(rng: &mut R, player_count: u16) -> Self {
        let entity_count = player_count as u32 * 20;
        let entity_type = Generate::generate(rng);
        let is_visible = rng.gen_bool(0.7);

        Self {
            damage: if is_visible && rng.gen_bool(0.6) {
                0
            } else {
                rng.gen_range(0..200)
            },
            entity_id: rng.gen_range(0..entity_count),
            entity_type: is_visible.then_some(entity_type),
            guidance: Guidance::generate(rng, entity_type),
            player_id: is_visible.then(|| rng.gen_range(0..player_count)),
            reloads: is_visible
                .then(|| {
                    let p = rng.gen_range(0.0..1.0);
                    (0..entity_type.weapon_count())
                        .map(|_| rng.gen_bool(p))
                        .collect()
                })
                .unwrap_or_default(),
            transform: Transform::generate(rng, entity_type),
            turret_angles: is_visible
                .then(|| {
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
    #[cfg_attr(feature = "bitcode", bitcode_hint(gamma))]
    chunk_id: (i8, i8),
    data: Vec<u8>,
}

impl TerrainUpdate {
    fn generate<R: Rng>(rng: &mut R, radius: i8) -> Self {
        let is_new = rng.gen_bool(0.02);

        Self {
            chunk_id: (
                rng.gen_range(-radius..radius),
                rng.gen_range(-radius..radius),
            ),
            data: generate_vec(rng, if is_new { 900..1100 } else { 3..9 }),
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
        let player_count = rng.gen_range(200..400) as u16;
        let world_radius = (player_count as f32 * 15000.0).sqrt() + rng.gen_range(-50.0..50.0);
        let terrain_radius = (world_radius / 1600.0).ceil() as i8;

        Self {
            contacts: (0..rng.gen_range(20..40))
                .map(|_| Contact::generate(rng, player_count))
                .collect(),
            score: (rng.gen::<f32>().powi(4) * 5000.0) as u32 + rng.gen_range(0..50),
            world_radius,
            terrain_updates: (0..rng.gen_range(3..10))
                .map(|_| TerrainUpdate::generate(rng, terrain_radius))
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
