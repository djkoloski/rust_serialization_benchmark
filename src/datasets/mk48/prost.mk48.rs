#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2f {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transform {
    #[prost(int32, tag = "1")]
    pub altitude: i32,
    #[prost(uint32, tag = "2")]
    pub angle: u32,
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<Vector2f>,
    #[prost(int32, tag = "4")]
    pub velocity: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Guidance {
    #[prost(uint32, tag = "1")]
    pub angle: u32,
    #[prost(bool, tag = "2")]
    pub submerge: bool,
    #[prost(int32, tag = "3")]
    pub velocity: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(uint32, tag = "1")]
    pub damage: u32,
    #[prost(uint32, tag = "2")]
    pub entity_id: u32,
    #[prost(enumeration = "EntityType", optional, tag = "3")]
    pub entity_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub guidance: ::core::option::Option<Guidance>,
    #[prost(uint32, optional, tag = "5")]
    pub player_id: ::core::option::Option<u32>,
    #[prost(bool, repeated, tag = "6")]
    pub reloads: ::prost::alloc::vec::Vec<bool>,
    #[prost(message, optional, tag = "7")]
    pub transform: ::core::option::Option<Transform>,
    #[prost(uint32, repeated, tag = "8")]
    pub turret_angles: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkId {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerrainUpdate {
    #[prost(message, optional, tag = "1")]
    pub chunk_id: ::core::option::Option<ChunkId>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    #[prost(uint32, tag = "2")]
    pub score: u32,
    #[prost(float, tag = "3")]
    pub world_radius: f32,
    #[prost(message, repeated, tag = "4")]
    pub terrain_updates: ::prost::alloc::vec::Vec<TerrainUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Updates {
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntityType {
    ArleighBurke = 0,
    Bismarck = 1,
    Clemenceau = 2,
    Fletcher = 3,
    G5 = 4,
    Iowa = 5,
    Kolkata = 6,
    Osa = 7,
    Yasen = 8,
    Zubr = 9,
}
impl EntityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EntityType::ArleighBurke => "ARLEIGH_BURKE",
            EntityType::Bismarck => "BISMARCK",
            EntityType::Clemenceau => "CLEMENCEAU",
            EntityType::Fletcher => "FLETCHER",
            EntityType::G5 => "G5",
            EntityType::Iowa => "IOWA",
            EntityType::Kolkata => "KOLKATA",
            EntityType::Osa => "OSA",
            EntityType::Yasen => "YASEN",
            EntityType::Zubr => "ZUBR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ARLEIGH_BURKE" => Some(Self::ArleighBurke),
            "BISMARCK" => Some(Self::Bismarck),
            "CLEMENCEAU" => Some(Self::Clemenceau),
            "FLETCHER" => Some(Self::Fletcher),
            "G5" => Some(Self::G5),
            "IOWA" => Some(Self::Iowa),
            "KOLKATA" => Some(Self::Kolkata),
            "OSA" => Some(Self::Osa),
            "YASEN" => Some(Self::Yasen),
            "ZUBR" => Some(Self::Zubr),
            _ => None,
        }
    }
}
