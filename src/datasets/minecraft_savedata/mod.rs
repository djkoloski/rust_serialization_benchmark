#[cfg(feature = "capnp")]
pub mod minecraft_savedata_capnp;
#[cfg(feature = "flatbuffers")]
#[path = "minecraft_savedata_generated.rs"]
#[allow(unused_imports, clippy::all)]
pub mod minecraft_savedata_fb;
#[cfg(feature = "prost")]
#[path = "prost.minecraft_savedata.rs"]
pub mod minecraft_savedata_prost;

#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(any(feature = "capnp", feature = "prost"))]
pub use minecraft_savedata_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use minecraft_savedata_fb::minecraft_savedata as fb;
#[cfg(feature = "prost")]
use minecraft_savedata_prost as pb;
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
pub enum GameType {
    #[cfg_attr(feature = "bilrost", bilrost(0))]
    Survival,
    #[cfg_attr(feature = "bilrost", bilrost(1))]
    Creative,
    #[cfg_attr(feature = "bilrost", bilrost(2))]
    Adventure,
    #[cfg_attr(feature = "bilrost", bilrost(3))]
    Spectator,
}

#[cfg(feature = "rkyv")]
unsafe impl rkyv::traits::NoUndef for ArchivedGameType {}

impl Generate for GameType {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        match rand.gen_range(0..4) {
            0 => GameType::Survival,
            1 => GameType::Creative,
            2 => GameType::Adventure,
            3 => GameType::Spectator,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl From<GameType> for fb::GameType {
    #[inline]
    fn from(value: GameType) -> Self {
        match value {
            GameType::Survival => fb::GameType::Survival,
            GameType::Creative => fb::GameType::Creative,
            GameType::Adventure => fb::GameType::Adventure,
            GameType::Spectator => fb::GameType::Spectator,
        }
    }
}

#[cfg(any(feature = "capnp", feature = "prost"))]
impl From<GameType> for cp::GameType {
    #[inline]
    fn from(value: GameType) -> Self {
        match value {
            GameType::Survival => cp::GameType::Survival,
            GameType::Creative => cp::GameType::Creative,
            GameType::Adventure => cp::GameType::Adventure,
            GameType::Spectator => cp::GameType::Spectator,
        }
    }
}

#[cfg(feature = "prost")]
impl From<GameType> for pb::GameType {
    #[inline]
    fn from(value: GameType) -> Self {
        match value {
            GameType::Survival => pb::GameType::Survival,
            GameType::Creative => pb::GameType::Creative,
            GameType::Adventure => pb::GameType::Adventure,
            GameType::Spectator => pb::GameType::Spectator,
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::GameType> for GameType {
    fn from(value: pb::GameType) -> Self {
        match value {
            pb::GameType::Survival => GameType::Survival,
            pb::GameType::Creative => GameType::Creative,
            pb::GameType::Adventure => GameType::Adventure,
            pb::GameType::Spectator => GameType::Spectator,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct Item {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(varint)))]
    #[cfg_attr(feature = "wiring", fixed(2))]
    pub count: i8,
    #[cfg_attr(feature = "bilrost", bilrost(encoding(varint)))]
    pub slot: u8,
    pub id: String,
}

impl Generate for Item {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&str; 8] = [
            "dirt",
            "stone",
            "pickaxe",
            "sand",
            "gravel",
            "shovel",
            "chestplate",
            "steak",
        ];
        Self {
            count: rng.gen(),
            slot: rng.gen(),
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Item {
    type Target = fb::Item<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let id = fbb.create_string(&self.id);

        let mut builder = fb::ItemBuilder::new(fbb);
        builder.add_count(self.count);
        builder.add_slot(self.slot);
        builder.add_id(id);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Item {
    type Reader = cp::item::Reader<'a>;
    type Builder = cp::item::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        use capnp::text::Reader;

        builder.set_count(self.count);
        builder.set_slot(self.slot);
        builder.set_id(Reader(self.id.as_bytes()));
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Item {
    type Message = pb::Item;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            count: self.count as i32,
            slot: self.slot as u32,
            id: self.id.clone(),
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::Item> for Item {
    fn from(value: pb::Item) -> Self {
        Item {
            count: value.count.try_into().unwrap(),
            slot: value.slot.try_into().unwrap(),
            id: value.id,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
pub struct Abilities {
    #[cfg_attr(feature = "wiring", fixed)]
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub may_fly: bool,
    pub flying: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub instabuild: bool,
}

impl Generate for Abilities {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        Self {
            walk_speed: rng.gen(),
            fly_speed: rng.gen(),
            may_fly: rng.gen_bool(0.5),
            flying: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            may_build: rng.gen_bool(0.5),
            instabuild: rng.gen_bool(0.5),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl From<Abilities> for fb::Abilities {
    #[inline]
    fn from(value: Abilities) -> Self {
        Self::new(
            value.walk_speed,
            value.fly_speed,
            value.may_fly,
            value.flying,
            value.invulnerable,
            value.may_build,
            value.instabuild,
        )
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Abilities {
    type Reader = cp::abilities::Reader<'a>;
    type Builder = cp::abilities::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_walk_speed(self.walk_speed);
        builder.set_fly_speed(self.fly_speed);
        builder.set_may_fly(self.may_fly);
        builder.set_flying(self.flying);
        builder.set_invulnerable(self.invulnerable);
        builder.set_may_build(self.may_build);
        builder.set_instabuild(self.instabuild);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Abilities {
    type Message = pb::Abilities;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            walk_speed: self.walk_speed,
            fly_speed: self.fly_speed,
            may_fly: self.may_fly,
            flying: self.flying,
            invulnerable: self.invulnerable,
            may_build: self.may_build,
            instabuild: self.instabuild,
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::Abilities> for Abilities {
    fn from(value: pb::Abilities) -> Self {
        Abilities {
            walk_speed: value.walk_speed,
            fly_speed: value.fly_speed,
            may_fly: value.may_fly,
            flying: value.flying,
            invulnerable: value.invulnerable,
            may_build: value.may_build,
            instabuild: value.instabuild,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct Entity {
    pub id: String,
    #[cfg_attr(feature = "wiring", fixed(11))]
    pub pos: (f64, f64, f64),
    pub motion: (f64, f64, f64),
    pub rotation: (f32, f32),
    pub fall_distance: f32,
    pub fire: u16,
    pub air: u16,
    pub on_ground: bool,
    pub no_gravity: bool,
    pub invulnerable: bool,
    pub portal_cooldown: i32,
    #[cfg_attr(feature = "bilrost", bilrost(encoding = "packed<fixed>"))]
    pub uuid: [u32; 4],
    pub custom_name: Option<String>,
    #[cfg_attr(feature = "wiring", fixed)]
    pub custom_name_visible: bool,
    pub silent: bool,
    pub glowing: bool,
}

impl Generate for Entity {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&str; 8] = [
            "cow", "sheep", "zombie", "skeleton", "spider", "creeper", "parrot", "bee",
        ];
        const CUSTOM_NAMES: [&str; 8] = [
            "rainbow", "princess", "steve", "johnny", "missy", "coward", "fairy", "howard",
        ];

        Self {
            id: IDS[rng.gen_range(0..IDS.len())].to_string(),
            pos: <(f64, f64, f64) as Generate>::generate(rng),
            motion: <(f64, f64, f64) as Generate>::generate(rng),
            rotation: <(f32, f32) as Generate>::generate(rng),
            fall_distance: rng.gen(),
            fire: rng.gen(),
            air: rng.gen(),
            on_ground: rng.gen_bool(0.5),
            no_gravity: rng.gen_bool(0.5),
            invulnerable: rng.gen_bool(0.5),
            portal_cooldown: rng.gen(),
            uuid: <[u32; 4] as Generate>::generate(rng),
            custom_name: <Option<()> as Generate>::generate(rng)
                .map(|_| CUSTOM_NAMES[rng.gen_range(0..CUSTOM_NAMES.len())].to_string()),
            custom_name_visible: rng.gen_bool(0.5),
            silent: rng.gen_bool(0.5),
            glowing: rng.gen_bool(0.5),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Entity {
    type Target = fb::Entity<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let id = fbb.create_string(&self.id);
        let custom_name = self
            .custom_name
            .as_ref()
            .map(|name| fbb.create_string(name));

        let pos = fb::Vector3d::new(self.pos.0, self.pos.1, self.pos.2);
        let motion = fb::Vector3d::new(self.motion.0, self.motion.1, self.motion.2);
        let rotation = fb::Vector2f::new(self.rotation.0, self.rotation.1);
        let uuid = fb::Uuid::new(self.uuid[0], self.uuid[1], self.uuid[2], self.uuid[3]);

        let mut builder = fb::EntityBuilder::new(fbb);
        builder.add_id(id);
        builder.add_pos(&pos);
        builder.add_motion(&motion);
        builder.add_rotation(&rotation);
        builder.add_fall_distance(self.fall_distance);
        builder.add_fire(self.fire);
        builder.add_air(self.air);
        builder.add_on_ground(self.on_ground);
        builder.add_no_gravity(self.no_gravity);
        builder.add_invulnerable(self.invulnerable);
        builder.add_portal_cooldown(self.portal_cooldown);
        builder.add_uuid(&uuid);
        if let Some(custom_name) = custom_name {
            builder.add_custom_name(custom_name);
        }
        builder.add_custom_name_visible(self.custom_name_visible);
        builder.add_silent(self.silent);
        builder.add_glowing(self.glowing);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Entity {
    type Reader = cp::entity::Reader<'a>;
    type Builder = cp::entity::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        use capnp::text::Reader;

        builder.set_id(Reader(self.id.as_bytes()));
        let mut pos = builder.reborrow().init_pos();
        pos.set_x(self.pos.0);
        pos.set_y(self.pos.1);
        pos.set_z(self.pos.2);
        let mut motion = builder.reborrow().init_motion();
        motion.set_x(self.motion.0);
        motion.set_y(self.motion.1);
        motion.set_z(self.motion.2);
        let mut rotation = builder.reborrow().init_rotation();
        rotation.set_x(self.rotation.0);
        rotation.set_y(self.rotation.1);
        builder.set_fall_distance(self.fall_distance);
        builder.set_fire(self.fire);
        builder.set_air(self.air);
        builder.set_on_ground(self.on_ground);
        builder.set_no_gravity(self.no_gravity);
        builder.set_invulnerable(self.invulnerable);
        builder.set_portal_cooldown(self.portal_cooldown);
        let mut uuid = builder.reborrow().init_uuid();
        uuid.set_x0(self.uuid[0]);
        uuid.set_x1(self.uuid[1]);
        uuid.set_x2(self.uuid[2]);
        uuid.set_x3(self.uuid[3]);
        if let Some(ref custom_name) = self.custom_name {
            builder.set_custom_name(Reader(custom_name.as_bytes()));
        }
        builder.set_custom_name_visible(self.custom_name_visible);
        builder.set_silent(self.silent);
        builder.set_glowing(self.glowing);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Entity {
    type Message = pb::Entity;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            id: self.id.clone(),
            pos: Some(pb::Vector3d {
                x: self.pos.0,
                y: self.pos.1,
                z: self.pos.2,
            }),
            motion: Some(pb::Vector3d {
                x: self.motion.0,
                y: self.motion.1,
                z: self.motion.2,
            }),
            rotation: Some(pb::Vector2f {
                x: self.rotation.0,
                y: self.rotation.1,
            }),
            fall_distance: self.fall_distance,
            fire: self.fire as u32,
            air: self.air as u32,
            on_ground: self.on_ground,
            no_gravity: self.no_gravity,
            invulnerable: self.invulnerable,
            portal_cooldown: self.portal_cooldown,
            uuid: Some(pb::Uuid {
                x0: self.uuid[0],
                x1: self.uuid[1],
                x2: self.uuid[2],
                x3: self.uuid[3],
            }),
            custom_name: self.custom_name.clone(),
            custom_name_visible: self.custom_name_visible,
            silent: self.silent,
            glowing: self.glowing,
        }
    }
}

#[cfg(feature = "prost")]
impl From<pb::Vector3d> for (f64, f64, f64) {
    fn from(value: pb::Vector3d) -> Self {
        (value.x, value.y, value.z)
    }
}

#[cfg(feature = "prost")]
impl From<pb::Vector2f> for (f32, f32) {
    fn from(value: pb::Vector2f) -> Self {
        (value.x, value.y)
    }
}

#[cfg(feature = "prost")]
impl From<pb::Uuid> for [u32; 4] {
    fn from(value: pb::Uuid) -> Self {
        [value.x0, value.x1, value.x2, value.x3]
    }
}

#[cfg(feature = "prost")]
impl From<pb::Entity> for Entity {
    fn from(value: pb::Entity) -> Self {
        Entity {
            id: value.id,
            pos: value.pos.unwrap().into(),
            motion: value.motion.unwrap().into(),
            rotation: value.rotation.unwrap().into(),
            fall_distance: value.fall_distance,
            fire: value.fire.try_into().unwrap(),
            air: value.air.try_into().unwrap(),
            on_ground: value.on_ground,
            no_gravity: value.no_gravity,
            invulnerable: value.invulnerable,
            portal_cooldown: value.portal_cooldown,
            uuid: value.uuid.unwrap().into(),
            custom_name: value.custom_name,
            custom_name_visible: value.custom_name_visible,
            silent: value.silent,
            glowing: value.glowing,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct RecipeBook {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub recipes: Vec<String>,
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub to_be_displayed: Vec<String>,
    #[cfg_attr(feature = "wiring", fixed)]
    pub is_filtering_craftable: bool,
    pub is_gui_open: bool,
    pub is_furnace_filtering_craftable: bool,
    pub is_furnace_gui_open: bool,
    pub is_blasting_furnace_filtering_craftable: bool,
    pub is_blasting_furnace_gui_open: bool,
    pub is_smoker_filtering_craftable: bool,
    pub is_smoker_gui_open: bool,
}

impl Generate for RecipeBook {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const RECIPES: [&str; 8] = [
            "pickaxe",
            "torch",
            "bow",
            "crafting table",
            "furnace",
            "shears",
            "arrow",
            "tnt",
        ];
        const MAX_RECIPES: usize = 30;
        const MAX_DISPLAYED_RECIPES: usize = 10;
        Self {
            recipes: generate_vec::<_, ()>(rng, 0..MAX_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            to_be_displayed: generate_vec::<_, ()>(rng, 0..MAX_DISPLAYED_RECIPES)
                .iter()
                .map(|_| RECIPES[rng.gen_range(0..RECIPES.len())].to_string())
                .collect(),
            is_filtering_craftable: rng.gen_bool(0.5),
            is_gui_open: rng.gen_bool(0.5),
            is_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_furnace_gui_open: rng.gen_bool(0.5),
            is_blasting_furnace_filtering_craftable: rng.gen_bool(0.5),
            is_blasting_furnace_gui_open: rng.gen_bool(0.5),
            is_smoker_filtering_craftable: rng.gen_bool(0.5),
            is_smoker_gui_open: rng.gen_bool(0.5),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for RecipeBook {
    type Target = fb::RecipeBook<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut recipes = Vec::new();
        for recipe in self.recipes.iter() {
            recipes.push(fbb.create_string(recipe));
        }
        let recipes = fbb.create_vector(&recipes);

        let mut to_be_displayed = Vec::new();
        for name in self.to_be_displayed.iter() {
            to_be_displayed.push(fbb.create_string(name));
        }
        let to_be_displayed = fbb.create_vector(&to_be_displayed);

        let mut builder = fb::RecipeBookBuilder::new(fbb);
        builder.add_recipes(recipes);
        builder.add_to_be_displayed(to_be_displayed);
        builder.add_is_filtering_craftable(self.is_filtering_craftable);
        builder.add_is_gui_open(self.is_gui_open);
        builder.add_is_furnace_filtering_craftable(self.is_furnace_filtering_craftable);
        builder.add_is_furnace_gui_open(self.is_furnace_gui_open);
        builder.add_is_blasting_furnace_filtering_craftable(
            self.is_blasting_furnace_filtering_craftable,
        );
        builder.add_is_blasting_furnace_gui_open(self.is_blasting_furnace_gui_open);
        builder.add_is_smoker_filtering_craftable(self.is_smoker_filtering_craftable);
        builder.add_is_smoker_gui_open(self.is_smoker_gui_open);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for RecipeBook {
    type Reader = cp::recipe_book::Reader<'a>;
    type Builder = cp::recipe_book::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        use capnp::text::Reader;

        let mut recipes = builder.reborrow().init_recipes(self.recipes.len() as u32);
        for (i, recipe) in self.recipes.iter().enumerate() {
            recipes.set(i as u32, Reader(recipe.as_bytes()));
        }
        let mut to_be_displayed = builder
            .reborrow()
            .init_to_be_displayed(self.to_be_displayed.len() as u32);
        for (i, name) in self.to_be_displayed.iter().enumerate() {
            to_be_displayed.set(i as u32, Reader(name.as_bytes()));
        }
        builder.set_is_filtering_craftable(self.is_filtering_craftable);
        builder.set_is_gui_open(self.is_gui_open);
        builder.set_is_furnace_filtering_craftable(self.is_furnace_filtering_craftable);
        builder.set_is_furnace_gui_open(self.is_furnace_gui_open);
        builder.set_is_blasting_furnace_filtering_craftable(
            self.is_blasting_furnace_filtering_craftable,
        );
        builder.set_is_blasting_furnace_gui_open(self.is_blasting_furnace_gui_open);
        builder.set_is_smoker_filtering_craftable(self.is_smoker_filtering_craftable);
        builder.set_is_smoker_gui_open(self.is_smoker_gui_open);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for RecipeBook {
    type Message = pb::RecipeBook;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for recipe in self.recipes.iter() {
            result.recipes.push(recipe.clone());
        }
        for tbd in self.to_be_displayed.iter() {
            result.to_be_displayed.push(tbd.clone());
        }
        result.is_filtering_craftable = self.is_filtering_craftable;
        result.is_gui_open = self.is_gui_open;
        result.is_furnace_filtering_craftable = self.is_furnace_filtering_craftable;
        result.is_furnace_gui_open = self.is_furnace_gui_open;
        result.is_blasting_furnace_filtering_craftable =
            self.is_blasting_furnace_filtering_craftable;
        result.is_blasting_furnace_gui_open = self.is_blasting_furnace_gui_open;
        result.is_smoker_filtering_craftable = self.is_smoker_filtering_craftable;
        result.is_smoker_gui_open = self.is_smoker_gui_open;
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::RecipeBook> for RecipeBook {
    fn from(value: pb::RecipeBook) -> Self {
        RecipeBook {
            recipes: value.recipes,
            to_be_displayed: value.to_be_displayed,
            is_filtering_craftable: value.is_filtering_craftable,
            is_gui_open: value.is_gui_open,
            is_furnace_filtering_craftable: value.is_furnace_filtering_craftable,
            is_furnace_gui_open: value.is_furnace_gui_open,
            is_blasting_furnace_filtering_craftable: value.is_blasting_furnace_filtering_craftable,
            is_blasting_furnace_gui_open: value.is_blasting_furnace_gui_open,
            is_smoker_filtering_craftable: value.is_smoker_filtering_craftable,
            is_smoker_gui_open: value.is_smoker_gui_open,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct Player {
    #[cfg_attr(feature = "wiring", fixed(3))]
    pub game_type: GameType,
    pub previous_game_type: GameType,
    pub score: i64,
    pub dimension: String,
    pub selected_item_slot: u32,
    pub selected_item: Item,
    pub spawn_dimension: Option<String>,
    #[cfg_attr(feature = "wiring", fixed(3))]
    pub spawn_x: i64,
    pub spawn_y: i64,
    pub spawn_z: i64,
    pub spawn_forced: Option<bool>,
    #[cfg_attr(feature = "wiring", fixed(8))]
    pub sleep_timer: u16,
    pub food_exhaustion_level: f32,
    pub food_saturation_level: f32,
    pub food_tick_timer: u32,
    pub xp_level: u32,
    pub xp_p: f32,
    pub xp_total: i32,
    pub xp_seed: i32,
    pub inventory: Vec<Item>,
    pub ender_items: Vec<Item>,
    pub abilities: Abilities,
    pub entered_nether_position: Option<(f64, f64, f64)>,
    #[cfg_attr(feature = "bilrost", bilrost(encoding = "(packed<fixed>, general)"))]
    pub root_vehicle: Option<([u32; 4], Entity)>,
    pub shoulder_entity_left: Option<Entity>,
    pub shoulder_entity_right: Option<Entity>,
    pub seen_credits: bool,
    pub recipe_book: RecipeBook,
}

impl Generate for Player {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const DIMENSIONS: [&str; 3] = ["overworld", "nether", "end"];
        const MAX_ITEMS: usize = 40;
        const MAX_ENDER_ITEMS: usize = 27;
        Self {
            game_type: GameType::generate(rng),
            previous_game_type: GameType::generate(rng),
            score: rng.gen(),
            dimension: DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string(),
            selected_item_slot: rng.gen(),
            selected_item: Item::generate(rng),
            spawn_dimension: <Option<()> as Generate>::generate(rng)
                .map(|_| DIMENSIONS[rng.gen_range(0..DIMENSIONS.len())].to_string()),
            spawn_x: rng.gen(),
            spawn_y: rng.gen(),
            spawn_z: rng.gen(),
            spawn_forced: <Option<bool> as Generate>::generate(rng),
            sleep_timer: rng.gen(),
            food_exhaustion_level: rng.gen(),
            food_saturation_level: rng.gen(),
            food_tick_timer: rng.gen(),
            xp_level: rng.gen(),
            xp_p: rng.gen(),
            xp_total: rng.gen(),
            xp_seed: rng.gen(),
            inventory: generate_vec(rng, 0..MAX_ITEMS),
            ender_items: generate_vec(rng, 0..MAX_ENDER_ITEMS),
            abilities: Abilities::generate(rng),
            entered_nether_position: <Option<(f64, f64, f64)> as Generate>::generate(rng),
            root_vehicle: <Option<([u32; 4], Entity)> as Generate>::generate(rng),
            shoulder_entity_left: <Option<Entity> as Generate>::generate(rng),
            shoulder_entity_right: <Option<Entity> as Generate>::generate(rng),
            seen_credits: rng.gen_bool(0.5),
            recipe_book: RecipeBook::generate(rng),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Player {
    type Target = fb::Player<'a>;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let dimension = fbb.create_string(&self.dimension);
        let selected_item = self.selected_item.serialize_fb(fbb);
        let spawn_dimension = self.spawn_dimension.as_ref().map(|d| fbb.create_string(d));

        let mut inventory = Vec::new();
        for inventory_item in self.inventory.iter() {
            inventory.push(inventory_item.serialize_fb(fbb));
        }
        let inventory = fbb.create_vector(&inventory);

        let mut ender_items = Vec::new();
        for ender_item in self.ender_items.iter() {
            ender_items.push(ender_item.serialize_fb(fbb));
        }
        let ender_items = fbb.create_vector(&ender_items);

        let abilities = self.abilities.into();
        let entered_nether_position = self
            .entered_nether_position
            .map(|p| fb::Vector3d::new(p.0, p.1, p.2));
        let root_vehicle = self.root_vehicle.as_ref().map(|v| {
            let entity = Some(v.1.serialize_fb(fbb));
            fb::Vehicle::create(
                fbb,
                &fb::VehicleArgs {
                    param_0: v.0[0],
                    param_1: v.0[1],
                    param_2: v.0[2],
                    param_3: v.0[3],
                    entity,
                },
            )
        });
        let shoulder_entity_left = self
            .shoulder_entity_left
            .as_ref()
            .map(|e| e.serialize_fb(fbb));
        let shoulder_entity_right = self
            .shoulder_entity_right
            .as_ref()
            .map(|e| e.serialize_fb(fbb));
        let recipe_book = self.recipe_book.serialize_fb(fbb);

        let mut builder = fb::PlayerBuilder::new(fbb);
        builder.add_game_type(self.game_type.into());
        builder.add_previous_game_type(self.previous_game_type.into());
        builder.add_score(self.score);
        builder.add_dimension(dimension);
        builder.add_selected_item_slot(self.selected_item_slot);
        builder.add_selected_item(selected_item);
        if let Some(spawn_dimension) = spawn_dimension {
            builder.add_spawn_dimension(spawn_dimension);
        }
        builder.add_spawn_x(self.spawn_x);
        builder.add_spawn_y(self.spawn_y);
        builder.add_spawn_z(self.spawn_z);
        builder.add_spawn_forced(self.spawn_forced.unwrap_or(false));
        builder.add_sleep_timer(self.sleep_timer);
        builder.add_food_exhaustion_level(self.food_exhaustion_level);
        builder.add_food_saturation_level(self.food_saturation_level);
        builder.add_food_tick_timer(self.food_tick_timer);
        builder.add_xp_level(self.xp_level);
        builder.add_xp_p(self.xp_p);
        builder.add_xp_total(self.xp_total);
        builder.add_xp_seed(self.xp_seed);
        builder.add_inventory(inventory);
        builder.add_ender_items(ender_items);
        builder.add_abilities(&abilities);
        if let Some(ref entered_nether_position) = entered_nether_position {
            builder.add_entered_nether_position(entered_nether_position);
        }
        if let Some(root_vehicle) = root_vehicle {
            builder.add_root_vehicle(root_vehicle);
        }
        if let Some(shoulder_entity_left) = shoulder_entity_left {
            builder.add_shoulder_entity_left(shoulder_entity_left);
        }
        if let Some(shoulder_entity_right) = shoulder_entity_right {
            builder.add_shoulder_entity_right(shoulder_entity_right);
        }
        builder.add_seen_credits(self.seen_credits);
        builder.add_recipe_book(recipe_book);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Player {
    type Reader = cp::player::Reader<'a>;
    type Builder = cp::player::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        use capnp::text::Reader;

        builder.set_game_type(self.game_type.into());
        builder.set_previous_game_type(self.previous_game_type.into());
        builder.set_score(self.score);
        builder.set_dimension(Reader(self.dimension.as_bytes()));
        let mut selected_item = builder.reborrow().init_selected_item();
        self.selected_item.serialize_capnp(&mut selected_item);
        let mut spawn_dimension = builder.reborrow().init_spawn_dimension();
        if let Some(ref value) = self.spawn_dimension {
            spawn_dimension.set_some(Reader(value.as_bytes()));
        } else {
            spawn_dimension.set_none(());
        }
        let mut spawn = builder.reborrow().init_spawn();
        spawn.set_x(self.spawn_x);
        spawn.set_y(self.spawn_y);
        spawn.set_z(self.spawn_z);
        let mut spawn_forced = builder.reborrow().init_spawn_forced();
        if let Some(ref value) = self.spawn_forced {
            spawn_forced.set_some(*value);
        } else {
            spawn_forced.set_none(());
        }
        builder.set_sleep_timer(self.sleep_timer);
        builder.set_food_exhaustion_level(self.food_exhaustion_level);
        builder.set_food_saturation_level(self.food_saturation_level);
        builder.set_food_tick_timer(self.food_tick_timer);
        builder.set_xp_level(self.xp_level);
        builder.set_xp_p(self.xp_p);
        builder.set_xp_total(self.xp_total);
        builder.set_xp_seed(self.xp_seed);
        let mut inventory = builder
            .reborrow()
            .init_inventory(self.inventory.len() as u32);
        for (i, value) in self.inventory.iter().enumerate() {
            value.serialize_capnp(&mut inventory.reborrow().get(i as u32));
        }
        let mut ender_items = builder
            .reborrow()
            .init_ender_items(self.ender_items.len() as u32);
        for (i, value) in self.ender_items.iter().enumerate() {
            value.serialize_capnp(&mut ender_items.reborrow().get(i as u32));
        }
        self.abilities
            .serialize_capnp(&mut builder.reborrow().init_abilities());
        let mut entered_nether_position = builder.reborrow().init_entered_nether_position();
        if let Some(ref value) = self.entered_nether_position {
            let mut builder = entered_nether_position.init_some();
            builder.set_x(value.0);
            builder.set_y(value.1);
            builder.set_z(value.2);
        } else {
            entered_nether_position.set_none(());
        }
        let mut root_vehicle = builder.reborrow().init_root_vehicle();
        if let Some(ref value) = self.root_vehicle {
            let mut builder = root_vehicle.init_some();
            let mut uuid = builder.reborrow().init_uuid();
            uuid.set_x0(value.0[0]);
            uuid.set_x1(value.0[1]);
            uuid.set_x2(value.0[2]);
            uuid.set_x3(value.0[3]);
            value
                .1
                .serialize_capnp(&mut builder.reborrow().init_entity());
        } else {
            root_vehicle.set_none(());
        }
        let mut shoulder_entity_left = builder.reborrow().init_shoulder_entity_left();
        if let Some(ref value) = self.shoulder_entity_left {
            value.serialize_capnp(&mut shoulder_entity_left.init_some());
        } else {
            shoulder_entity_left.set_none(());
        }
        let mut shoulder_entity_right = builder.reborrow().init_shoulder_entity_right();
        if let Some(ref value) = self.shoulder_entity_right {
            value.serialize_capnp(&mut shoulder_entity_right.init_some());
        } else {
            shoulder_entity_right.set_none(());
        }
        builder.set_seen_credits(self.seen_credits);
        self.recipe_book
            .serialize_capnp(&mut builder.reborrow().init_recipe_book());
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Player {
    type Message = pb::Player;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message {
            game_type: pb::GameType::from(self.game_type) as i32,
            previous_game_type: self.previous_game_type as i32,
            score: self.score,
            dimension: self.dimension.clone(),
            selected_item_slot: self.selected_item_slot,
            selected_item: Some(self.selected_item.serialize_pb()),
            spawn_dimension: self.spawn_dimension.clone(),
            spawn_x: self.spawn_x,
            spawn_y: self.spawn_y,
            spawn_z: self.spawn_z,
            spawn_forced: self.spawn_forced,
            sleep_timer: self.sleep_timer as u32,
            food_exhaustion_level: self.food_exhaustion_level,
            food_saturation_level: self.food_saturation_level,
            food_tick_timer: self.food_tick_timer,
            xp_level: self.xp_level,
            xp_p: self.xp_p,
            xp_total: self.xp_total,
            xp_seed: self.xp_seed,
            inventory: Default::default(),
            ender_items: Default::default(),
            abilities: Some(self.abilities.serialize_pb()),
            entered_nether_position: self.entered_nether_position.map(|p| pb::Vector3d {
                x: p.0,
                y: p.1,
                z: p.2,
            }),
            root_vehicle: self.root_vehicle.as_ref().map(|v| pb::Vehicle {
                uuid: Some(pb::Uuid {
                    x0: v.0[0],
                    x1: v.0[1],
                    x2: v.0[2],
                    x3: v.0[3],
                }),
                entity: Some(v.1.serialize_pb()),
            }),
            shoulder_entity_left: self.shoulder_entity_left.as_ref().map(|e| e.serialize_pb()),
            shoulder_entity_right: self
                .shoulder_entity_right
                .as_ref()
                .map(|e| e.serialize_pb()),
            seen_credits: self.seen_credits,
            recipe_book: Some(self.recipe_book.serialize_pb()),
        };
        for item in self.inventory.iter() {
            result.inventory.push(item.serialize_pb());
        }
        for item in self.ender_items.iter() {
            result.ender_items.push(item.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::Player> for Player {
    fn from(value: pb::Player) -> Self {
        Player {
            game_type: pb::GameType::try_from(value.game_type).unwrap().into(),
            previous_game_type: pb::GameType::try_from(value.previous_game_type)
                .unwrap()
                .into(),
            score: value.score,
            dimension: value.dimension,
            selected_item_slot: value.selected_item_slot,
            selected_item: value.selected_item.unwrap().into(),
            spawn_dimension: value.spawn_dimension,
            spawn_x: value.spawn_x,
            spawn_y: value.spawn_y,
            spawn_z: value.spawn_z,
            spawn_forced: value.spawn_forced,
            sleep_timer: value.sleep_timer.try_into().unwrap(),
            food_exhaustion_level: value.food_exhaustion_level,
            food_saturation_level: value.food_saturation_level,
            food_tick_timer: value.food_tick_timer,
            xp_level: value.xp_level,
            xp_p: value.xp_p,
            xp_total: value.xp_total,
            xp_seed: value.xp_seed,
            inventory: value.inventory.into_iter().map(Into::into).collect(),
            ender_items: value.ender_items.into_iter().map(Into::into).collect(),
            abilities: value.abilities.unwrap().into(),
            entered_nether_position: value.entered_nether_position.map(Into::into),
            root_vehicle: value
                .root_vehicle
                .map(|vehicle| (vehicle.uuid.unwrap().into(), vehicle.entity.unwrap().into())),
            shoulder_entity_left: value.shoulder_entity_left.map(Into::into),
            shoulder_entity_right: value.shoulder_entity_right.map(Into::into),
            seen_credits: value.seen_credits,
            recipe_book: value.recipe_book.unwrap().into(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct Players {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub players: Vec<Player>,
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Players {
    type Target = fb::Players<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut players = Vec::new();
        for player in self.players.iter() {
            players.push(player.serialize_fb(fbb));
        }
        let players = fbb.create_vector(&players);

        let mut builder = fb::PlayersBuilder::new(fbb);
        builder.add_players(players);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Players {
    type Reader = cp::players::Reader<'a>;
    type Builder = cp::players::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut players = builder.reborrow().init_players(self.players.len() as u32);
        for (i, value) in self.players.iter().enumerate() {
            value.serialize_capnp(&mut players.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Players {
    type Message = pb::Players;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for player in self.players.iter() {
            result.players.push(player.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<pb::Players> for Players {
    fn from(value: pb::Players) -> Self {
        Players {
            players: value.players.into_iter().map(Into::into).collect(),
        }
    }
}
