//! The bilrost implementation for the minecraft savedata types is customized purely to support the
//! UUID type and the (f64, f64, f64) native tuple type. If those were changed no conversion would
//! be necessary.

use bilrost::Message;

use crate::bench_bilrost::ToBilrost;

#[derive(Clone, PartialEq, Message)]
pub struct Vector3d(pub f64, pub f64, pub f64);

impl From<(f64, f64, f64)> for Vector3d {
    fn from(value: (f64, f64, f64)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<Vector3d> for (f64, f64, f64) {
    fn from(value: Vector3d) -> Self {
        (value.0, value.1, value.2)
    }
}

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

/// Bilrost has its own Entity
#[derive(Clone, PartialEq, Message)]
pub struct Entity {
    pub id: String,
    pub pos: Vector3d,
    pub motion: Vector3d,
    pub rotation: Vector2f,
    pub fall_distance: f32,
    pub fire: u16,
    pub air: u16,
    pub on_ground: bool,
    pub no_gravity: bool,
    pub invulnerable: bool,
    pub portal_cooldown: i32,
    #[bilrost(encoding(packed < fixed >))]
    pub uuid: Vec<u32>,
    pub custom_name: Option<String>,
    pub custom_name_visible: bool,
    pub silent: bool,
    pub glowing: bool,
}

impl ToBilrost for super::Entity {
    type Message = Entity;
    type Serializable<'a> = Entity;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Entity {
            id: self.id.clone(),
            pos: self.pos.into(),
            motion: self.motion.into(),
            rotation: self.rotation.into(),
            fall_distance: self.fall_distance,
            fire: self.fire,
            air: self.air,
            on_ground: self.on_ground,
            no_gravity: self.no_gravity,
            invulnerable: self.invulnerable,
            portal_cooldown: self.portal_cooldown,
            uuid: self.uuid.into(),
            custom_name: self.custom_name.clone(),
            custom_name_visible: self.custom_name_visible,
            silent: self.silent,
            glowing: self.glowing,
        }
    }
}

impl From<Entity> for super::Entity {
    fn from(value: Entity) -> Self {
        Self {
            id: value.id,
            pos: value.pos.into(),
            motion: value.motion.into(),
            rotation: value.rotation.into(),
            fall_distance: value.fall_distance,
            fire: value.fire,
            air: value.air,
            on_ground: value.on_ground,
            no_gravity: value.no_gravity,
            invulnerable: value.invulnerable,
            portal_cooldown: value.portal_cooldown,
            uuid: value.uuid.as_slice().try_into().unwrap(),
            custom_name: value.custom_name,
            custom_name_visible: value.custom_name_visible,
            silent: value.silent,
            glowing: value.glowing,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Vehicle {
    #[bilrost(encoding(packed < fixed >))]
    pub uuid: Vec<u32>,
    pub entity: Entity,
}

#[derive(Clone, PartialEq, Message)]
pub struct Player {
    pub game_type: Option<super::GameType>,
    pub previous_game_type: Option<super::GameType>,
    pub score: i64,
    pub dimension: String,
    pub selected_item_slot: u32,
    pub selected_item: super::Item,
    pub spawn_dimension: Option<String>,
    pub spawn_x: i64,
    pub spawn_y: i64,
    pub spawn_z: i64,
    pub spawn_forced: Option<bool>,
    pub sleep_timer: u16,
    pub food_exhaustion_level: f32,
    pub food_saturation_level: f32,
    pub food_tick_timer: u32,
    pub xp_level: u32,
    pub xp_p: f32,
    pub xp_total: i32,
    pub xp_seed: i32,
    #[bilrost(encoding(packed))]
    pub inventory: Vec<super::Item>,
    #[bilrost(encoding(packed))]
    pub ender_items: Vec<super::Item>,
    pub abilities: super::Abilities,
    pub entered_nether_position: Option<Vector3d>,
    pub root_vehicle: Option<Vehicle>,
    pub shoulder_entity_left: Option<Entity>,
    pub shoulder_entity_right: Option<Entity>,
    pub seen_credits: bool,
    pub recipe_book: super::RecipeBook,
}

impl ToBilrost for super::Player {
    type Message = Player;
    type Serializable<'a> = Player;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Player {
            game_type: Some(self.game_type),
            previous_game_type: Some(self.previous_game_type),
            score: self.score,
            dimension: self.dimension.clone(),
            selected_item_slot: self.selected_item_slot,
            selected_item: self.selected_item.clone(),
            spawn_dimension: self.spawn_dimension.clone(),
            spawn_x: self.spawn_x,
            spawn_y: self.spawn_y,
            spawn_z: self.spawn_z,
            spawn_forced: self.spawn_forced,
            sleep_timer: self.sleep_timer,
            food_exhaustion_level: self.food_exhaustion_level,
            food_saturation_level: self.food_saturation_level,
            food_tick_timer: self.food_tick_timer,
            xp_level: self.xp_level,
            xp_p: self.xp_p,
            xp_total: self.xp_total,
            xp_seed: self.xp_seed,
            inventory: self.inventory.clone(),
            ender_items: self.ender_items.clone(),
            abilities: self.abilities,
            entered_nether_position: self.entered_nether_position.map(Into::into),
            root_vehicle: self.root_vehicle.as_ref().map(|(uuid, entity)| Vehicle {
                uuid: uuid.into(),
                entity: entity.to_bilrost(),
            }),
            shoulder_entity_left: self
                .shoulder_entity_left
                .as_ref()
                .map(ToBilrost::to_bilrost),
            shoulder_entity_right: self
                .shoulder_entity_right
                .as_ref()
                .map(ToBilrost::to_bilrost),
            seen_credits: self.seen_credits,
            recipe_book: self.recipe_book.clone(),
        }
    }
}

impl From<Player> for super::Player {
    fn from(value: Player) -> Self {
        Self {
            game_type: value.game_type.unwrap(),
            previous_game_type: value.previous_game_type.unwrap(),
            score: value.score,
            dimension: value.dimension,
            selected_item_slot: value.selected_item_slot,
            selected_item: value.selected_item,
            spawn_dimension: value.spawn_dimension,
            spawn_x: value.spawn_x,
            spawn_y: value.spawn_y,
            spawn_z: value.spawn_z,
            spawn_forced: value.spawn_forced,
            sleep_timer: value.sleep_timer,
            food_exhaustion_level: value.food_exhaustion_level,
            food_saturation_level: value.food_saturation_level,
            food_tick_timer: value.food_tick_timer,
            xp_level: value.xp_level,
            xp_p: value.xp_p,
            xp_total: value.xp_total,
            xp_seed: value.xp_seed,
            inventory: value.inventory,
            ender_items: value.ender_items,
            abilities: value.abilities,
            entered_nether_position: value.entered_nether_position.map(Into::into),
            root_vehicle: value.root_vehicle.map(|vehicle| {
                (
                    vehicle.uuid.as_slice().try_into().unwrap(),
                    vehicle.entity.into(),
                )
            }),
            shoulder_entity_left: value.shoulder_entity_left.map(Into::into),
            shoulder_entity_right: value.shoulder_entity_right.map(Into::into),
            seen_credits: value.seen_credits,
            recipe_book: value.recipe_book,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Players {
    #[bilrost(encoding(packed))]
    pub players: Vec<Player>,
}

impl ToBilrost for super::Players {
    type Message = Players;
    type Serializable<'a> = Players;

    fn to_bilrost(&self) -> Self::Serializable<'_> {
        Players {
            players: self.players.iter().map(ToBilrost::to_bilrost).collect(),
        }
    }
}

impl From<Players> for super::Players {
    fn from(value: Players) -> Self {
        Self {
            players: value.players.into_iter().map(Into::into).collect(),
        }
    }
}
