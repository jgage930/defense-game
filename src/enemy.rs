use bevy::prelude::*;
use rand::Rng;
use std::default::Default;

use crate::commons::GameTextures;
use crate::wall::Wall;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemy)
            .add_system(movement)
            .add_system(animate_enemy_sprite)
            .add_system(state_transitions);
    }
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    health: f32,
}

impl Enemy {
    pub fn apply_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn get_hurtbox() -> Vec2 {
        Vec2::new(57.5, 82.5)
    }

    fn health(&self) -> f32 {
        self.health
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 50.,
            health: 100.,
        }
    }
}

#[derive(Component)]
enum EnemyState {
    Walk,
    Death,
    Attack,
}

#[derive(Resource, Deref, DerefMut)]
pub struct EnemySpawnTimer(pub Timer);

impl EnemySpawnTimer {
}

fn spawn_enemy(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    if spawn_timer.finished() {
        // pick random y
        let y = rand::thread_rng().gen_range(-385..385) as f32;
        // spawn enemy
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_textures.enemy_walk.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(57.5, 82.5)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-800., y, 100.),
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(Enemy::default())
        .insert(EnemyState::Walk);
    }

    spawn_timer.tick(time.delta());
}

// move based on state
fn movement(
    mut query: Query<(&Enemy, &EnemyState, &mut Transform)>,
    time: Res<Time>,
) {
    for (enemy, enemy_state, mut transform) in query.iter_mut() {
        let dx = match enemy_state {
            EnemyState::Walk => {
                enemy.speed * time.delta_seconds()
            },
            _ => 0.
        };
        transform.translation.x += dx;
    }
}

// logic for changing state based on current state
fn state_transitions(
    mut query: Query<(Entity, &mut EnemyState, &mut Enemy, &Transform)>,
) {
    for (_enemy_entity, mut enemy_state, enemy, transform) in query.iter_mut() {
        if transform.translation.x >= Wall::LEFT - Wall::SIZE {
            *enemy_state = EnemyState::Attack;
        }

        if enemy.health() <= 0. {
            *enemy_state = EnemyState::Death;
        }
    }
}

// Logic for animation
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_enemy_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &EnemyState, &mut AnimationTimer, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
    game_textures: Res<GameTextures>,
    mut wall: ResMut<Wall>,
) {
    for (entity, enemy_state, mut timer, mut sprite, mut texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        // Set enemy animations based on state
        match enemy_state {
            EnemyState::Walk => {
                if sprite.index >= 12 {
                    sprite.index = 0;
                }
                let atlas = game_textures.enemy_walk.clone();
                *texture_atlas_handle = atlas;

                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            },
            EnemyState::Death => {
                if sprite.index >= 10 {
                    sprite.index = 0;
                }
                let atlas = game_textures.enemy_death.clone();
                *texture_atlas_handle = atlas;

                if timer.just_finished() {
                    if sprite.index >= 9 {
                        commands.entity(entity).despawn();
                    } else {
                        sprite.index += 1;
                    }
                }
            },
            EnemyState::Attack => {
                if sprite.index >= 17 {
                    sprite.index = 0;
                    wall.apply_damage(10.);
                }
                
                let atlas = game_textures.enemy_attack.clone();
                *texture_atlas_handle = atlas;


                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
        };
    }
}