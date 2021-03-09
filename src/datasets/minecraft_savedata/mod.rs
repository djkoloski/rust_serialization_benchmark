mod minecraft_savedata_generated;

use core::pin::Pin;
use crate::{bench_flatbuffers, generate_vec, Generate};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
pub use minecraft_savedata_generated::minecraft_savedata as fb;
use rand::Rng;
use rkyv::Archived;

#[derive(
    Clone, Copy,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
)]
#[archive(copy)]
#[repr(u8)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

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

impl Into<fb::GameType> for GameType {
    fn into(self) -> fb::GameType {
        match self {
            GameType::Survival => fb::GameType::Survival,
            GameType::Creative => fb::GameType::Creative,
            GameType::Adventure => fb::GameType::Adventure,
            GameType::Spectator => fb::GameType::Spectator,
        }
    }
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Item {
    pub count: i8,
    pub slot: u8,
    pub id: String,
}

impl Generate for Item {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&'static str; 8] = [
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

impl<'a> bench_flatbuffers::Serialize<'a> for Item {
    type Target = fb::Item<'a>;

    fn serialize<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let id = Some(builder.create_string(&self.id));
        Self::Target::create(builder, &fb::ItemArgs {
            count: self.count,
            slot: self.slot,
            id,
        })
    }
}

#[derive(
    Clone, Copy,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
)]
#[archive(copy)]
pub struct Abilities {
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

impl Into<fb::Abilities> for Abilities {
    fn into(self) -> fb::Abilities {
        fb::Abilities::new(
            self.walk_speed,
            self.fly_speed,
            self.may_fly,
            self.flying,
            self.invulnerable,
            self.may_build,
            self.instabuild,
        )
    }
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Entity {
    pub id: String,
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
    pub uuid: [u32; 4],
    pub custom_name: Option<String>,
    pub custom_name_visible: bool,
    pub silent: bool,
    pub glowing: bool,
}

impl Generate for Entity {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const IDS: [&'static str; 8] = [
            "cow", "sheep", "zombie", "skeleton", "spider", "creeper", "parrot", "bee",
        ];
        const CUSTOM_NAMES: [&'static str; 8] = [
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

impl<'a> bench_flatbuffers::Serialize<'a> for Entity {
    type Target = fb::Entity<'a>;

    fn serialize<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
    'a: 'b,
    {
        let id = Some(builder.create_string(&self.id));
        let custom_name = self.custom_name.as_ref().map(|name| builder.create_string(&name));
        Self::Target::create(builder, &fb::EntityArgs {
            id,
            pos: Some(&fb::Vector3d::new(self.pos.0, self.pos.1, self.pos.2)),
            motion: Some(&fb::Vector3d::new(self.motion.0, self.motion.1, self.motion.2)),
            rotation: Some(&fb::Vector2f::new(self.rotation.0, self.rotation.1)),
            fall_distance: self.fall_distance,
            fire: self.fire,
            air: self.air,
            on_ground: self.on_ground,
            no_gravity: self.no_gravity,
            invulnerable: self.invulnerable,
            portal_cooldown: self.portal_cooldown,
            uuid: Some(&fb::Uuid::new(self.uuid[0], self.uuid[1], self.uuid[2], self.uuid[3])),
            custom_name,
            custom_name_visible: self.custom_name_visible,
            silent: self.silent,
            glowing: self.glowing,
        })
    }
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct RecipeBook {
    pub recipes: Vec<String>,
    pub to_be_displayed: Vec<String>,
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
        const RECIPES: [&'static str; 8] = [
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

impl<'a> bench_flatbuffers::Serialize<'a> for RecipeBook {
    type Target = fb::RecipeBook<'a>;

    fn serialize<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut recipes = Vec::new();
        for recipe in self.recipes.iter() {
            recipes.push(builder.create_string(recipe));
        }
        let recipes = Some(builder.create_vector(&recipes));

        let mut to_be_displayed = Vec::new();
        for name in self.to_be_displayed.iter() {
            to_be_displayed.push(builder.create_string(name));
        }
        let to_be_displayed = Some(builder.create_vector(&to_be_displayed));

        Self::Target::create(builder, &fb::RecipeBookArgs {
            recipes,
            to_be_displayed,
            is_filtering_craftable: self.is_filtering_craftable,
            is_gui_open: self.is_gui_open,
            is_furnace_filtering_craftable: self.is_furnace_filtering_craftable,
            is_furnace_gui_open: self.is_furnace_gui_open,
            is_blasting_furnace_filtering_craftable: self.is_blasting_furnace_filtering_craftable,
            is_blasting_furnace_gui_open: self.is_blasting_furnace_gui_open,
            is_smoker_filtering_craftable: self.is_smoker_filtering_craftable,
            is_smoker_gui_open: self.is_smoker_gui_open,
        })
    }
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Player {
    pub game_type: GameType,
    pub previous_game_type: GameType,
    pub score: u64,
    pub dimension: String,
    pub selected_item_slot: u32,
    pub selected_item: Item,
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
    pub inventory: Vec<Item>,
    pub ender_items: Vec<Item>,
    pub abilities: Abilities,
    pub entered_nether_position: Option<(f64, f64, f64)>,
    pub root_vehicle: Option<([u32; 4], Entity)>,
    pub shoulder_entity_left: Option<Entity>,
    pub shoulder_entity_right: Option<Entity>,
    pub seen_credits: bool,
    pub recipe_book: RecipeBook,
}

impl ArchivedPlayer {
    pub fn game_type_pin(self: Pin<&mut Self>) -> Pin<&mut GameType> {
        unsafe { self.map_unchecked_mut(|s| &mut s.game_type) }
    }

    pub fn spawn_x_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_x) }
    }

    pub fn spawn_y_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_y) }
    }

    pub fn spawn_z_pin(self: Pin<&mut Self>) -> Pin<&mut i64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.spawn_z) }
    }
}

impl Generate for Player {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        const DIMENSIONS: [&'static str; 3] = ["overworld", "nether", "end"];
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

impl<'a> bench_flatbuffers::Serialize<'a> for Player {
    type Target = fb::Player<'a>;

    fn serialize<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let dimension = Some(builder.create_string(&self.dimension));
        let selected_item = Some(self.selected_item.serialize(builder));
        let spawn_dimension = self.spawn_dimension.as_ref().map(|d| builder.create_string(d));

        let mut inventory = Vec::new();
        for inventory_item in self.inventory.iter() {
            inventory.push(inventory_item.serialize(builder));
        }
        let inventory = Some(builder.create_vector(&inventory));

        let mut ender_items = Vec::new();
        for ender_item in self.ender_items.iter() {
            ender_items.push(ender_item.serialize(builder));
        }
        let ender_items = Some(builder.create_vector(&ender_items));

        let entered_nether_position = self.entered_nether_position.map(|p| {
            fb::Vector3d::new(p.0, p.1, p.2)
        });
        let root_vehicle = self.root_vehicle.as_ref().map(|v| {
            let entity = Some(v.1.serialize(builder));
            fb::Vehicle::create(builder, &fb::VehicleArgs {
                param_0: v.0[0],
                param_1: v.0[1],
                param_2: v.0[2],
                param_3: v.0[3],
                entity,
            })
        });
        let shoulder_entity_left = self.shoulder_entity_left.as_ref().map(|e| e.serialize(builder));
        let shoulder_entity_right = self.shoulder_entity_right.as_ref().map(|e| e.serialize(builder));
        let recipe_book = Some(self.recipe_book.serialize(builder));

        Self::Target::create(builder, &fb::PlayerArgs {
            game_type: self.game_type.into(),
            previous_game_type: self.previous_game_type.into(),
            score: self.score,
            dimension,
            selected_item_slot: self.selected_item_slot,
            selected_item,
            spawn_dimension,
            spawn_x: self.spawn_x,
            spawn_y: self.spawn_y,
            spawn_z: self.spawn_z,
            spawn_forced: self.spawn_forced.unwrap_or(false),
            sleep_timer: self.sleep_timer,
            food_exhaustion_level: self.food_exhaustion_level,
            food_saturation_level: self.food_saturation_level,
            food_tick_timer: self.food_tick_timer,
            xp_level: self.xp_level,
            xp_p: self.xp_p,
            xp_total: self.xp_total,
            xp_seed: self.xp_seed,
            inventory,
            ender_items,
            abilities: Some(&self.abilities.into()),
            entered_nether_position: entered_nether_position.as_ref(),
            root_vehicle,
            shoulder_entity_left,
            shoulder_entity_right,
            seen_credits: self.seen_credits,
            recipe_book,
        })
    }
}

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Deserialize, serde::Serialize
)]
pub struct Players {
    pub players: Vec<Player>,
}

impl ArchivedPlayers {
    pub fn players_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Player>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.players) }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Players {
    type Target = fb::Players<'a>;

    fn serialize<'b>(&self, builder: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut players = Vec::new();
        for player in self.players.iter() {
            players.push(player.serialize(builder));
        }
        let players = Some(builder.create_vector(&players));
        fb::Players::create(builder, &fb::PlayersArgs {
            players,
        })
    }
}
