// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(int32, tag = "1")]
    pub count: i32,
    #[prost(uint32, tag = "2")]
    pub slot: u32,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Abilities {
    #[prost(float, tag = "1")]
    pub walk_speed: f32,
    #[prost(float, tag = "2")]
    pub fly_speed: f32,
    #[prost(bool, tag = "3")]
    pub may_fly: bool,
    #[prost(bool, tag = "4")]
    pub flying: bool,
    #[prost(bool, tag = "5")]
    pub invulnerable: bool,
    #[prost(bool, tag = "6")]
    pub may_build: bool,
    #[prost(bool, tag = "7")]
    pub instabuild: bool,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Vector3d {
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
    #[prost(double, tag = "3")]
    pub z: f64,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Vector2f {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(uint32, tag = "1")]
    pub x0: u32,
    #[prost(uint32, tag = "2")]
    pub x1: u32,
    #[prost(uint32, tag = "3")]
    pub x2: u32,
    #[prost(uint32, tag = "4")]
    pub x3: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pos: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "3")]
    pub motion: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "4")]
    pub rotation: ::core::option::Option<Vector2f>,
    #[prost(float, tag = "5")]
    pub fall_distance: f32,
    #[prost(uint32, tag = "6")]
    pub fire: u32,
    #[prost(uint32, tag = "7")]
    pub air: u32,
    #[prost(bool, tag = "8")]
    pub on_ground: bool,
    #[prost(bool, tag = "9")]
    pub no_gravity: bool,
    #[prost(bool, tag = "10")]
    pub invulnerable: bool,
    #[prost(int32, tag = "11")]
    pub portal_cooldown: i32,
    #[prost(message, optional, tag = "12")]
    pub uuid: ::core::option::Option<Uuid>,
    #[prost(string, optional, tag = "13")]
    pub custom_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag = "14")]
    pub custom_name_visible: bool,
    #[prost(bool, tag = "15")]
    pub silent: bool,
    #[prost(bool, tag = "16")]
    pub glowing: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecipeBook {
    #[prost(string, repeated, tag = "1")]
    pub recipes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub to_be_displayed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub is_filtering_craftable: bool,
    #[prost(bool, tag = "4")]
    pub is_gui_open: bool,
    #[prost(bool, tag = "5")]
    pub is_furnace_filtering_craftable: bool,
    #[prost(bool, tag = "6")]
    pub is_furnace_gui_open: bool,
    #[prost(bool, tag = "7")]
    pub is_blasting_furnace_filtering_craftable: bool,
    #[prost(bool, tag = "8")]
    pub is_blasting_furnace_gui_open: bool,
    #[prost(bool, tag = "9")]
    pub is_smoker_filtering_craftable: bool,
    #[prost(bool, tag = "10")]
    pub is_smoker_gui_open: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    #[prost(message, optional, tag = "1")]
    pub uuid: ::core::option::Option<Uuid>,
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(enumeration = "GameType", tag = "1")]
    pub game_type: i32,
    #[prost(enumeration = "GameType", tag = "2")]
    pub previous_game_type: i32,
    #[prost(int64, tag = "3")]
    pub score: i64,
    #[prost(string, tag = "4")]
    pub dimension: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub selected_item_slot: u32,
    #[prost(message, optional, tag = "6")]
    pub selected_item: ::core::option::Option<Item>,
    #[prost(string, optional, tag = "7")]
    pub spawn_dimension: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, tag = "8")]
    pub spawn_x: i64,
    #[prost(int64, tag = "9")]
    pub spawn_y: i64,
    #[prost(int64, tag = "10")]
    pub spawn_z: i64,
    #[prost(bool, optional, tag = "11")]
    pub spawn_forced: ::core::option::Option<bool>,
    #[prost(uint32, tag = "12")]
    pub sleep_timer: u32,
    #[prost(float, tag = "13")]
    pub food_exhaustion_level: f32,
    #[prost(float, tag = "14")]
    pub food_saturation_level: f32,
    #[prost(uint32, tag = "15")]
    pub food_tick_timer: u32,
    #[prost(uint32, tag = "16")]
    pub xp_level: u32,
    #[prost(float, tag = "17")]
    pub xp_p: f32,
    #[prost(int32, tag = "18")]
    pub xp_total: i32,
    #[prost(int32, tag = "19")]
    pub xp_seed: i32,
    #[prost(message, repeated, tag = "20")]
    pub inventory: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, repeated, tag = "21")]
    pub ender_items: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, optional, tag = "22")]
    pub abilities: ::core::option::Option<Abilities>,
    #[prost(message, optional, tag = "23")]
    pub entered_nether_position: ::core::option::Option<Vector3d>,
    #[prost(message, optional, tag = "24")]
    pub root_vehicle: ::core::option::Option<Vehicle>,
    #[prost(message, optional, tag = "25")]
    pub shoulder_entity_left: ::core::option::Option<Entity>,
    #[prost(message, optional, tag = "26")]
    pub shoulder_entity_right: ::core::option::Option<Entity>,
    #[prost(bool, tag = "27")]
    pub seen_credits: bool,
    #[prost(message, optional, tag = "28")]
    pub recipe_book: ::core::option::Option<RecipeBook>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Players {
    #[prost(message, repeated, tag = "1")]
    pub players: ::prost::alloc::vec::Vec<Player>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameType {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}
impl GameType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Survival => "SURVIVAL",
            Self::Creative => "CREATIVE",
            Self::Adventure => "ADVENTURE",
            Self::Spectator => "SPECTATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SURVIVAL" => Some(Self::Survival),
            "CREATIVE" => Some(Self::Creative),
            "ADVENTURE" => Some(Self::Adventure),
            "SPECTATOR" => Some(Self::Spectator),
            _ => None,
        }
    }
}
