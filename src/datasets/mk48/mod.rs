#[cfg(any(feature = "capnp", feature = "prost"))]
pub mod mk48_capnp;
#[cfg(feature = "flatbuffers")]
#[path = "mk48_generated.rs"]
#[allow(unused_imports, clippy::all)]
pub mod mk48_fb;
#[cfg(feature = "prost")]
#[path = "prost.mk48.rs"]
pub mod mk48_prost;

#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(any(feature = "capnp", feature = "prost"))]
pub use mk48_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use mk48_fb::mk_48 as fb;
#[cfg(feature = "prost")]
use mk48_prost as pb;
#[cfg(feature = "nanoserde")]
use nanoserde::{DeBin, SerBin};
use rand::Rng;
#[cfg(feature = "wiring")]
use wiring::prelude::{Unwiring, Wiring};

#[cfg(feature = "capnp")]
use crate::bench_capnp;
#[cfg(feature = "flatbuffers")]
use crate::bench_flatbuffers;
#[cfg(feature = "prost")]
use crate::bench_prost;
use crate::{generate_vec, Generate};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Enumeration))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring), tag(u8))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
#[repr(u8)]
pub enum EntityType {
    #[cfg_attr(feature = "bilrost", bilrost(0))]
    ArleighBurke,
    #[cfg_attr(feature = "bilrost", bilrost(1))]
    Bismarck,
    #[cfg_attr(feature = "bilrost", bilrost(2))]
    Clemenceau,
    #[cfg_attr(feature = "bilrost", bilrost(3))]
    Fletcher,
    #[cfg_attr(feature = "bilrost", bilrost(4))]
    G5,
    #[cfg_attr(feature = "bilrost", bilrost(5))]
    Iowa,
    #[cfg_attr(feature = "bilrost", bilrost(6))]
    Kolkata,
    #[cfg_attr(feature = "bilrost", bilrost(7))]
    Osa,
    #[cfg_attr(feature = "bilrost", bilrost(8))]
    Yasen,
    #[cfg_attr(feature = "bilrost", bilrost(9))]
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

#[cfg(feature = "flatbuffers")]
impl From<EntityType> for fb::EntityType {
    #[inline]
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::ArleighBurke => fb::EntityType::ArleighBurke,
            EntityType::Bismarck => fb::EntityType::Bismarck,
            EntityType::Clemenceau => fb::EntityType::Clemenceau,
            EntityType::Fletcher => fb::EntityType::Fletcher,
            EntityType::G5 => fb::EntityType::G5,
            EntityType::Iowa => fb::EntityType::Iowa,
            EntityType::Kolkata => fb::EntityType::Kolkata,
            EntityType::Osa => fb::EntityType::Osa,
            EntityType::Yasen => fb::EntityType::Yasen,
            EntityType::Zubr => fb::EntityType::Zubr,
        }
    }
}

#[cfg(any(feature = "capnp", feature = "prost"))]
impl From<EntityType> for cp::EntityType {
    #[inline]
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::ArleighBurke => cp::EntityType::ArleighBurke,
            EntityType::Bismarck => cp::EntityType::Bismarck,
            EntityType::Clemenceau => cp::EntityType::Clemenceau,
            EntityType::Fletcher => cp::EntityType::Fletcher,
            EntityType::G5 => cp::EntityType::G5,
            EntityType::Iowa => cp::EntityType::Iowa,
            EntityType::Kolkata => cp::EntityType::Kolkata,
            EntityType::Osa => cp::EntityType::Osa,
            EntityType::Yasen => cp::EntityType::Yasen,
            EntityType::Zubr => cp::EntityType::Zubr,
        }
    }
}

#[cfg(feature = "prost")]
impl From<EntityType> for pb::EntityType {
    #[inline]
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::ArleighBurke => pb::EntityType::ArleighBurke,
            EntityType::Bismarck => pb::EntityType::Bismarck,
            EntityType::Clemenceau => pb::EntityType::Clemenceau,
            EntityType::Fletcher => pb::EntityType::Fletcher,
            EntityType::G5 => pb::EntityType::G5,
            EntityType::Iowa => pb::EntityType::Iowa,
            EntityType::Kolkata => pb::EntityType::Kolkata,
            EntityType::Osa => pb::EntityType::Osa,
            EntityType::Yasen => pb::EntityType::Yasen,
            EntityType::Zubr => pb::EntityType::Zubr,
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::EntityType> for EntityType {
    fn from(value: pb::EntityType) -> Self {
        match value {
            pb::EntityType::ArleighBurke => EntityType::ArleighBurke,
            pb::EntityType::Bismarck => EntityType::Bismarck,
            pb::EntityType::Clemenceau => EntityType::Clemenceau,
            pb::EntityType::Fletcher => EntityType::Fletcher,
            pb::EntityType::G5 => EntityType::G5,
            pb::EntityType::Iowa => EntityType::Iowa,
            pb::EntityType::Kolkata => EntityType::Kolkata,
            pb::EntityType::Osa => EntityType::Osa,
            pb::EntityType::Yasen => EntityType::Yasen,
            pb::EntityType::Zubr => EntityType::Zubr,
        }
    }
}

fn generate_submerge(rng: &mut impl Rng, entity_type: EntityType) -> bool {
    entity_type.is_sub() && rng.gen_bool(0.9)
}

fn generate_altitude(rng: &mut impl Rng, entity_type: EntityType) -> i8 {
    if generate_submerge(rng, entity_type) {
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

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]

pub struct Transform {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(varint)))]
    #[cfg_attr(feature = "wiring", fixed)]
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

#[cfg(feature = "flatbuffers")]
impl From<Transform> for fb::Transform {
    fn from(value: Transform) -> Self {
        fb::Transform::new(
            value.altitude,
            value.angle,
            &fb::Vector2f::new(value.position.0, value.position.1),
            value.velocity,
        )
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Transform {
    type Reader = cp::transform::Reader<'a>;
    type Builder = cp::transform::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_altitude(self.altitude);
        builder.set_angle(self.angle);
        let mut position = builder.reborrow().init_position();
        position.set_x(self.position.0);
        position.set_y(self.position.1);
        builder.set_velocity(self.velocity);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Transform {
    type Message = pb::Transform;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            altitude: self.altitude.into(),
            angle: self.angle.into(),
            position: Some(pb::Vector2f {
                x: self.position.0,
                y: self.position.1,
            }),
            velocity: self.velocity.into(),
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::Vector2f> for (f32, f32) {
    fn from(value: pb::Vector2f) -> Self {
        (value.x, value.y)
    }
}

#[cfg(feature = "prost")]
impl From<pb::Transform> for Transform {
    fn from(value: pb::Transform) -> Self {
        Transform {
            altitude: value.altitude.try_into().unwrap(),
            angle: value.angle.try_into().unwrap(),
            position: value.position.unwrap().into(),
            velocity: value.velocity.try_into().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]

pub struct Guidance {
    #[cfg_attr(feature = "wiring", fixed)]
    pub angle: u16,
    pub submerge: bool,
    pub velocity: i16,
}

impl Guidance {
    fn generate<R: Rng>(rng: &mut R, entity_type: EntityType) -> Self {
        Self {
            angle: rng.gen(),
            submerge: generate_submerge(rng, entity_type),
            velocity: generate_velocity(rng),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl From<Guidance> for fb::Guidance {
    #[inline]
    fn from(value: Guidance) -> Self {
        Self::new(value.angle, value.submerge, value.velocity)
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Guidance {
    type Reader = cp::guidance::Reader<'a>;
    type Builder = cp::guidance::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_angle(self.angle);
        builder.set_submerge(self.submerge);
        builder.set_velocity(self.velocity);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Guidance {
    type Message = pb::Guidance;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            angle: self.angle as u32,
            submerge: self.submerge,
            velocity: self.velocity as i32,
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::Guidance> for Guidance {
    fn from(value: pb::Guidance) -> Self {
        Guidance {
            angle: value.angle.try_into().unwrap(),
            submerge: value.submerge,
            velocity: value.velocity.try_into().unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
pub struct Contact {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(varint)))]
    #[cfg_attr(feature = "wiring", fixed(2))]
    pub damage: u8,
    pub entity_id: u32,
    pub entity_type: Option<EntityType>,

    pub guidance: Guidance,
    pub player_id: Option<u16>,
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub reloads: Vec<bool>,
    pub transform: Transform,
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
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

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Contact {
    type Target = fb::Contact<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let reloads = fbb.create_vector_from_iter(self.reloads.iter());
        let turret_angles = fbb.create_vector_from_iter(self.turret_angles.iter());

        let mut builder = fb::ContactBuilder::new(fbb);
        builder.add_damage(self.damage);
        builder.add_entity_id(self.entity_id);
        if let Some(entity_type) = self.entity_type {
            builder.add_entity_type(entity_type.into());
        }
        builder.add_guidance(&self.guidance.into());
        if let Some(player_id) = self.player_id {
            builder.add_player_id(player_id);
        }
        builder.add_reloads(reloads);
        builder.add_transform(&self.transform.into());
        builder.add_turret_angles(turret_angles);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Contact {
    type Reader = cp::contact::Reader<'a>;
    type Builder = cp::contact::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_damage(self.damage);
        builder.set_entity_id(self.entity_id);
        let mut entity_type = builder.reborrow().init_entity_type();
        if let Some(value) = self.entity_type {
            entity_type.set_some(value.into());
        } else {
            entity_type.set_none(());
        }
        self.guidance
            .serialize_capnp(&mut builder.reborrow().init_guidance());
        let mut player_id = builder.reborrow().init_player_id();
        if let Some(value) = self.player_id {
            player_id.set_some(value);
        } else {
            player_id.set_none(());
        }
        let mut reloads = builder.reborrow().init_reloads(self.reloads.len() as u32);
        for (i, value) in self.reloads.iter().cloned().enumerate() {
            reloads.set(i as u32, value);
        }
        self.transform
            .serialize_capnp(&mut builder.reborrow().init_transform());
        let mut turret_angles = builder
            .reborrow()
            .init_turret_angles(self.turret_angles.len() as u32);
        for (i, value) in self.turret_angles.iter().cloned().enumerate() {
            turret_angles.set(i as u32, value);
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Contact {
    type Message = pb::Contact;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message {
            damage: self.damage as u32,
            entity_id: self.entity_id,
            entity_type: self
                .entity_type
                .map(|entity_type| cp::EntityType::from(entity_type) as i32),
            guidance: Some(self.guidance.serialize_pb()),
            player_id: self.player_id.map(Into::into),
            reloads: Default::default(),
            transform: Some(self.transform.serialize_pb()),
            turret_angles: Default::default(),
        };
        for reload in self.reloads.iter().cloned() {
            result.reloads.push(reload);
        }
        for turret_angle in self.turret_angles.iter().cloned() {
            result.turret_angles.push(turret_angle as u32);
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::Contact> for Contact {
    fn from(value: pb::Contact) -> Self {
        Contact {
            damage: value.damage.try_into().unwrap(),
            entity_id: value.entity_id,
            entity_type: value
                .entity_type
                .map(|et| <pb::EntityType>::try_from(et).unwrap().into()),
            guidance: value.guidance.unwrap().into(),
            player_id: value.player_id.map(|id| id.try_into().unwrap()),
            reloads: value.reloads,
            transform: value.transform.unwrap().into(),
            turret_angles: value
                .turret_angles
                .into_iter()
                .map(|a| a.try_into().unwrap())
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
pub struct TerrainUpdate {
    #[cfg_attr(feature = "bilrost", bilrost(encoding = "(varint, varint)"))]
    chunk_id: (i8, i8),
    #[cfg_attr(feature = "bilrost", bilrost(encoding = "plainbytes"))]
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

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for TerrainUpdate {
    type Target = fb::TerrainUpdate<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let data = fbb.create_vector_from_iter(self.data.iter());

        let mut builder = fb::TerrainUpdateBuilder::new(fbb);
        builder.add_chunk_id(&fb::ChunkId::new(self.chunk_id.0, self.chunk_id.1));
        builder.add_data(data);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for TerrainUpdate {
    type Reader = cp::terrain_update::Reader<'a>;
    type Builder = cp::terrain_update::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut chunk_id = builder.reborrow().init_chunk_id();
        chunk_id.set_x(self.chunk_id.0);
        chunk_id.set_y(self.chunk_id.1);
        let mut data = builder.reborrow().init_data(self.data.len() as u32);
        for (i, value) in self.data.iter().cloned().enumerate() {
            data.set(i as u32, value);
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for TerrainUpdate {
    type Message = pb::TerrainUpdate;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message {
            chunk_id: Some(pb::ChunkId {
                x: self.chunk_id.0 as i32,
                y: self.chunk_id.1 as i32,
            }),
            data: Default::default(),
        };
        for datum in self.data.iter().cloned() {
            result.data.push(datum);
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::ChunkId> for (i8, i8) {
    fn from(value: pb::ChunkId) -> Self {
        (value.x.try_into().unwrap(), value.y.try_into().unwrap())
    }
}

#[cfg(feature = "prost")]
impl From<pb::TerrainUpdate> for TerrainUpdate {
    fn from(value: pb::TerrainUpdate) -> Self {
        TerrainUpdate {
            chunk_id: value.chunk_id.unwrap().into(),
            data: value.data,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]

pub struct Update {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub contacts: Vec<Contact>,
    #[cfg_attr(feature = "wiring", fixed(2))]
    pub score: u32,
    pub world_radius: f32,
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
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

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Update {
    type Target = fb::Update<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut contacts = Vec::new();
        for contact in self.contacts.iter() {
            contacts.push(contact.serialize_fb(fbb));
        }
        let contacts = fbb.create_vector(&contacts);

        let mut terrain_updates = Vec::new();
        for terrain_update in self.terrain_updates.iter() {
            terrain_updates.push(terrain_update.serialize_fb(fbb));
        }
        let terrain_updates = fbb.create_vector(&terrain_updates);

        let mut builder = fb::UpdateBuilder::new(fbb);
        builder.add_contacts(contacts);
        builder.add_score(self.score);
        builder.add_world_radius(self.world_radius);
        builder.add_terrain_updates(terrain_updates);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Update {
    type Reader = cp::update::Reader<'a>;
    type Builder = cp::update::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut contacts = builder.reborrow().init_contacts(self.contacts.len() as u32);
        for (i, value) in self.contacts.iter().enumerate() {
            value.serialize_capnp(&mut contacts.reborrow().get(i as u32));
        }
        builder.set_score(self.score);
        builder.set_world_radius(self.world_radius);
        let mut terrain_updates = builder
            .reborrow()
            .init_terrain_updates(self.terrain_updates.len() as u32);
        for (i, value) in self.terrain_updates.iter().enumerate() {
            value.serialize_capnp(&mut terrain_updates.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Update {
    type Message = pb::Update;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for contact in self.contacts.iter() {
            result.contacts.push(contact.serialize_pb());
        }
        result.score = self.score;
        result.world_radius = self.world_radius;
        for terrain_update in self.terrain_updates.iter() {
            result.terrain_updates.push(terrain_update.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::Update> for Update {
    fn from(value: pb::Update) -> Self {
        Update {
            contacts: value.contacts.into_iter().map(Into::into).collect(),
            score: value.score,
            world_radius: value.world_radius,
            terrain_updates: value.terrain_updates.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
#[derive(Debug)]
pub struct Updates {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub updates: Vec<Update>,
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Updates {
    type Target = fb::Updates<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut updates = Vec::new();
        for update in self.updates.iter() {
            updates.push(update.serialize_fb(fbb));
        }
        let updates = fbb.create_vector(&updates);

        let mut builder = fb::UpdatesBuilder::new(fbb);
        builder.add_updates(updates);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Updates {
    type Reader = cp::updates::Reader<'a>;
    type Builder = cp::updates::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut updates = builder.reborrow().init_updates(self.updates.len() as u32);
        for (i, value) in self.updates.iter().enumerate() {
            value.serialize_capnp(&mut updates.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Updates {
    type Message = pb::Updates;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for update in self.updates.iter() {
            result.updates.push(update.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::Updates> for Updates {
    fn from(value: pb::Updates) -> Self {
        Updates {
            updates: value.updates.into_iter().map(Into::into).collect(),
        }
    }
}
