#[cfg(feature = "bebop")]
pub mod mk48_bebop;
#[cfg(feature = "capnp")]
pub mod mk48_capnp;
#[cfg(feature = "flatbuffers")]
#[path = "mk48_generated.rs"]
#[allow(unused_imports)]
pub mod mk48_fb;
#[cfg(feature = "prost")]
pub mod mk48_prost {
    include!(concat!(env!("OUT_DIR"), "/prost.mk48.rs"));
}

#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(feature = "capnp")]
pub use mk48_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use mk48_fb::mk_48 as fb;
#[cfg(feature = "prost")]
use mk48_prost as pb;
use rand::Rng;
#[cfg(feature = "rkyv")]
use rkyv::Archived;

#[cfg(feature = "capnp")]
use crate::bench_capnp;
#[cfg(feature = "flatbuffers")]
use crate::bench_flatbuffers;
#[cfg(feature = "prost")]
use crate::bench_prost;
use crate::{generate_vec, Generate};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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

#[cfg(feature = "flatbuffers")]
impl Into<fb::EntityType> for EntityType {
    #[inline]
    fn into(self) -> fb::EntityType {
        match self {
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

#[cfg(feature = "capnp")]
impl Into<cp::EntityType> for EntityType {
    #[inline]
    fn into(self) -> cp::EntityType {
        match self {
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
impl Into<pb::EntityType> for EntityType {
    #[inline]
    fn into(self) -> pb::EntityType {
        match self {
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

#[cfg(feature = "alkahest")]
impl alkahest::Pack<EntityType> for EntityType {
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<EntityType>, usize) {
        use EntityType::*;
        match self {
            ArleighBurke => EntityTypeArleighBurkePack.pack(offset, output),
            Bismarck => EntityTypeBismarckPack.pack(offset, output),
            Clemenceau => EntityTypeClemenceauPack.pack(offset, output),
            Fletcher => EntityTypeFletcherPack.pack(offset, output),
            G5 => EntityTypeG5Pack.pack(offset, output),
            Iowa => EntityTypeIowaPack.pack(offset, output),
            Kolkata => EntityTypeKolkataPack.pack(offset, output),
            Osa => EntityTypeOsaPack.pack(offset, output),
            Yasen => EntityTypeYasenPack.pack(offset, output),
            Zubr => EntityTypeZubrPack.pack(offset, output),
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

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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

#[cfg(feature = "flatbuffers")]
impl Into<fb::Transform> for Transform {
    fn into(self) -> fb::Transform {
        fb::Transform::new(
            self.altitude,
            self.angle,
            &fb::Vector2f::new(self.position.0, self.position.1),
            self.velocity,
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
        let mut result = Self::Message::default();
        result.altitude = self.altitude.into();
        result.angle = self.angle.into();
        result.position = Some({
            let mut result = pb::Vector2f::default();
            result.x = self.position.0;
            result.y = self.position.1;
            result
        });
        result.velocity = self.velocity.into();
        result
    }
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<Transform> for Transform {
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<Transform>, usize) {
        TransformPack {
            altitude: self.altitude,
            angle: self.angle,
            position: self.position,
            velocity: self.velocity,
        }
        .pack(offset, output)
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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
pub struct Guidance {
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
impl Into<fb::Guidance> for Guidance {
    #[inline]
    fn into(self) -> fb::Guidance {
        fb::Guidance::new(self.angle, self.submerge, self.velocity)
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
        builder.set_velocity(self.velocity.into());
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Guidance {
    type Message = pb::Guidance;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.angle = self.angle as u32;
        result.submerge = self.submerge;
        result.velocity = self.velocity as i32;
        result
    }
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<Guidance> for Guidance {
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<Guidance>, usize) {
        GuidancePack {
            angle: self.angle,
            submerge: self.submerge,
            velocity: self.velocity,
        }
        .pack(offset, output)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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
        let mut result = Self::Message::default();
        result.damage = self.damage as u32;
        result.entity_id = self.entity_id;
        result.entity_type = self
            .entity_type
            .map(|entity_type| <EntityType as Into<cp::EntityType>>::into(entity_type) as i32);
        result.guidance = Some(self.guidance.serialize_pb());
        result.player_id = self.player_id.map(Into::into);
        for reload in self.reloads.iter().cloned() {
            result.reloads.push(reload);
        }
        result.transform = Some(self.transform.serialize_pb());
        for turret_angle in self.turret_angles.iter().cloned() {
            result.turret_angles.push(turret_angle as u32);
        }
        result
    }
}

#[cfg(feature = "alkahest")]
#[derive(alkahest::Schema)]
pub struct ContactSchema {
    pub damage: u8,
    pub entity_id: u32,
    pub entity_type: Option<EntityType>,
    pub guidance: Guidance,
    pub player_id: Option<u16>,
    pub reloads: alkahest::Seq<bool>,
    pub transform: Transform,
    pub turret_angles: alkahest::Seq<u16>,
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<ContactSchema> for &'_ Contact {
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<ContactSchema>, usize) {
        ContactSchemaPack {
            damage: self.damage,
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            guidance: self.guidance,
            player_id: self.player_id,
            reloads: self.reloads.iter(),
            transform: self.transform,
            turret_angles: self.turret_angles.iter(),
        }
        .pack(offset, output)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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
        let mut result = Self::Message::default();
        result.chunk_id = {
            let mut result = pb::ChunkId::default();
            result.x = self.chunk_id.0 as i32;
            result.y = self.chunk_id.1 as i32;
            Some(result)
        };
        for datum in self.data.iter().cloned() {
            result.data.push(datum);
        }
        result
    }
}

#[cfg(feature = "alkahest")]
#[derive(alkahest::Schema)]
pub struct TerrainUpdateSchema {
    pub chunk_id: (i8, i8),
    pub data: alkahest::Bytes,
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<TerrainUpdateSchema> for &'_ TerrainUpdate {
    #[inline]
    fn pack(
        self,
        offset: usize,
        output: &mut [u8],
    ) -> (alkahest::Packed<TerrainUpdateSchema>, usize) {
        TerrainUpdateSchemaPack {
            chunk_id: self.chunk_id,
            data: self.data.iter(),
        }
        .pack(offset, output)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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

#[cfg(feature = "rkyv")]
const _: () = {
    use core::pin::Pin;

    impl ArchivedUpdate {
        pub fn score_pin(self: Pin<&mut Self>) -> Pin<&mut u32> {
            unsafe { self.map_unchecked_mut(|s| &mut s.score) }
        }
    }
};

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

#[cfg(feature = "alkahest")]
#[derive(alkahest::Schema)]
pub struct UpdateSchema {
    pub contacts: alkahest::Seq<ContactSchema>,
    pub score: u32,
    pub world_radius: f32,
    pub terrain_updates: alkahest::Seq<TerrainUpdateSchema>,
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<UpdateSchema> for &'_ Update {
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<UpdateSchema>, usize) {
        UpdateSchemaPack {
            contacts: self.contacts.iter(),
            score: self.score,
            world_radius: self.world_radius,
            terrain_updates: self.terrain_updates.iter(),
        }
        .pack(offset, output)
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "abomonation", derive(abomonation_derive::Abomonation))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
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

#[cfg(feature = "alkahest")]
#[derive(alkahest::Schema)]
pub struct UpdatesSchema {
    pub updates: alkahest::Seq<UpdateSchema>,
}

#[cfg(feature = "alkahest")]
impl alkahest::Pack<UpdatesSchema> for &'_ Updates {
    fn pack(self, offset: usize, output: &mut [u8]) -> (alkahest::Packed<UpdatesSchema>, usize) {
        UpdatesSchemaPack {
            updates: self.updates.iter(),
        }
        .pack(offset, output)
    }
}
