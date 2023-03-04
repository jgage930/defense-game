use bevy::prelude::*;
use std::time::Duration;

use crate::enemy::EnemySpawnTimer;

// Resources
#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub fireball: Handle<Image>,
    pub enemy_walk: Handle<TextureAtlas>,
    pub enemy_idle: Handle<TextureAtlas>,
    pub enemy_death: Handle<TextureAtlas>,
    pub enemy_attack: Handle<TextureAtlas>,
    pub wall: Handle<Image>,
}

// Sprite Paths
pub const PLAYER_SPRITE: &str = "player.png";
pub const FIREBALL_SPRITE: &str = "fireball.png";
pub const ENEMY_WALK: &str = "enemy/enemy_walk.png";
pub const ENEMY_IDLE: &str = "enemy/enemy_idle.png";
pub const ENEMY_DEATH: &str = "enemy/enemy_death.png";
pub const ENEMY_ATTACK: &str = "enemy/enemy_attack.png";
pub const WALL: &str = "wall.png";

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load in game texture atlases
    let enemy_walk_handle: Handle<Image> = asset_server.load(ENEMY_WALK);
    let enemy_idle_handle: Handle<Image> = asset_server.load(ENEMY_IDLE);
    let enemy_death_handle: Handle<Image> = asset_server.load(ENEMY_DEATH);
    let enemy_attack_handle: Handle<Image> = asset_server.load(ENEMY_ATTACK);
    let wall_handle: Handle<Image> = asset_server.load(WALL);

    let enemy_walk_atlas =
        TextureAtlas::from_grid(enemy_walk_handle, Vec2::new(22., 32.), 13, 1, None, None);

    let enemy_idle_atlas =
        TextureAtlas::from_grid(enemy_idle_handle, Vec2::new(24., 32.), 15, 1, None, None);

    let enemy_death_atlas =
        TextureAtlas::from_grid(enemy_death_handle, Vec2::new(33., 32.), 11, 1, None, None);

    let enemy_attack_atlas =
        TextureAtlas::from_grid(enemy_attack_handle, Vec2::new(43., 37.), 18, 1, None, None);

    let enemy_walk_atlas_handle = texture_atlases.add(enemy_walk_atlas);
    let enemy_idle_atlas_handle = texture_atlases.add(enemy_idle_atlas);
    let enemy_death_atlas_handle = texture_atlases.add(enemy_death_atlas);
    let enemy_attack_atlas_handle = texture_atlases.add(enemy_attack_atlas);

    // Load in game textures
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        fireball: asset_server.load(FIREBALL_SPRITE),
        enemy_walk: enemy_walk_atlas_handle,
        enemy_idle: enemy_idle_atlas_handle,
        enemy_death: enemy_death_atlas_handle,
        enemy_attack: enemy_attack_atlas_handle,
        wall: wall_handle,
    };
    commands.insert_resource(game_textures);

    commands.insert_resource(EnemySpawnTimer(Timer::new(
        Duration::from_secs(2),
        TimerMode::Repeating,
    )));
}

