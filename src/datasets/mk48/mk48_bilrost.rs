//! The bilrost implementation for the MK48 data types, as for minecraft_savedata, is customized
//! purely to support the fields which have native tuple types like (i8, i8) and (f32, f32). If
//! those were changed no conversion would be necessary.

use crate::bench_bilrost::ToBilrost;
use bilrost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct Vector2f(pub f32, pub f32);

impl From<(f32, f32)> for Vector2f {
    fn from(value: (f32, f32)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<Vector2f> for (f32, f32) {
    fn from(value: Vector2f) -> Self {
        (value.0, value.1)
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Transform {
    #[bilrost(encoding(varint))]
    pub altitude: i8,
    pub angle: u16,
    pub position: Vector2f,
    pub velocity: i16,
}

impl ToBilrost for super::Transform {
    type Message = Transform;
    type Serializable<'a> = Transform;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Transform {
            altitude: self.altitude,
            angle: self.angle,
            position: self.position.into(),
            velocity: self.velocity,
        }
    }
}

impl From<Transform> for super::Transform {
    fn from(value: Transform) -> Self {
        Self {
            altitude: value.altitude,
            angle: value.angle,
            position: value.position.into(),
            velocity: value.velocity,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Contact {
    #[bilrost(encoding(varint))]
    pub damage: u8,
    pub entity_id: u32,
    pub entity_type: Option<super::EntityType>,
    pub guidance: super::Guidance,
    pub player_id: Option<u16>,
    #[bilrost(encoding(packed))]
    pub reloads: Vec<bool>,
    pub transform: Transform,
    #[bilrost(encoding(packed))]
    pub turret_angles: Vec<u16>,
}

impl ToBilrost for super::Contact {
    type Message = Contact;
    type Serializable<'a> = Contact;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Contact {
            damage: self.damage,
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            guidance: self.guidance,
            player_id: self.player_id,
            reloads: self.reloads.clone(),
            transform: self.transform.to_bilrost(),
            turret_angles: self.turret_angles.clone(),
        }
    }
}

impl From<Contact> for super::Contact {
    fn from(value: Contact) -> Self {
        Self {
            damage: value.damage,
            entity_id: value.entity_id,
            entity_type: value.entity_type,
            guidance: value.guidance,
            player_id: value.player_id,
            reloads: value.reloads,
            transform: value.transform.into(),
            turret_angles: value.turret_angles,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct ChunkId {
    #[bilrost(encoding(varint))]
    pub x: i8,
    #[bilrost(encoding(varint))]
    pub y: i8,
}

impl From<(i8, i8)> for ChunkId {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<ChunkId> for (i8, i8) {
    fn from(value: ChunkId) -> Self {
        (value.x, value.y)
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct TerrainUpdate {
    pub chunk_id: ChunkId,
    #[bilrost(encoding(plainbytes))]
    pub data: Vec<u8>,
}

impl ToBilrost for super::TerrainUpdate {
    type Message = TerrainUpdate;
    type Serializable<'a> = TerrainUpdate;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        TerrainUpdate {
            chunk_id: self.chunk_id.into(),
            data: self.data.clone(),
        }
    }
}

impl From<TerrainUpdate> for super::TerrainUpdate {
    fn from(value: TerrainUpdate) -> Self {
        Self {
            chunk_id: value.chunk_id.into(),
            data: value.data,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Update {
    #[bilrost(encoding(packed))]
    pub contacts: Vec<Contact>,
    pub score: u32,
    pub world_radius: f32,
    #[bilrost(encoding(packed))]
    pub terrain_updates: Vec<TerrainUpdate>,
}

impl ToBilrost for super::Update {
    type Message = Update;
    type Serializable<'a> = Update;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Update {
            contacts: self.contacts.iter().map(ToBilrost::to_bilrost).collect(),
            score: self.score,
            world_radius: self.world_radius,
            terrain_updates: self
                .terrain_updates
                .iter()
                .map(ToBilrost::to_bilrost)
                .collect(),
        }
    }
}

impl From<Update> for super::Update {
    fn from(value: Update) -> Self {
        Self {
            contacts: value.contacts.into_iter().map(Into::into).collect(),
            score: value.score,
            world_radius: value.world_radius,
            terrain_updates: value.terrain_updates.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Updates {
    #[bilrost(encoding(packed))]
    pub updates: Vec<Update>,
}

impl ToBilrost for super::Updates {
    type Message = Updates;
    type Serializable<'a> = Updates;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Updates {
            updates: self.updates.iter().map(ToBilrost::to_bilrost).collect(),
        }
    }
}

impl From<Updates> for super::Updates {
    fn from(value: Updates) -> Self {
        Self {
            updates: value.updates.into_iter().map(Into::into).collect(),
        }
    }
}
