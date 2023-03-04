use bevy::prelude::*;
use std::default::Default;

use crate::commons::GameTextures;
use crate::enemy::{Enemy, EnemyState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(movement_player)
            .add_system(shoot)
            .add_system(movement_projectile)
            .add_system(add_wealth_system);
    }
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    wealth: usize,
}

impl Player {
    pub fn wealth(&self) -> usize {
        self.wealth
    }

    pub fn add_wealth(&mut self, amount: usize) {
        self.wealth += amount;
    }

    pub fn spend(&mut self, amount: usize) {
        self.wealth -= amount;
    }
}

fn spawn_player(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands
        .spawn(SpriteBundle {
            texture: game_textures.player.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(80., 80.)),

                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(750., 100., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 300.,
            wealth: 0,
        });
}

fn movement_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    let y = transform.translation.y;
    let dy = player.speed * time.delta_seconds();

    if keyboard.pressed(KeyCode::Up) {
        if y + dy + 40. < 385. {
            transform.translation.y += dy;
        }
    }

    if keyboard.pressed(KeyCode::Down) {
        if y - dy - 40. > -385. {
            transform.translation.y -= dy;
        }
    }
}

// Logic for shooting
// Plan to have several spells, but that is a task for later
#[derive(Component)]
pub struct Projectile {
    speed: f32,
    damage: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            speed: 500.,
            damage: 20.,
        }
    }
}

impl Projectile {
    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub fn get_hitbox() -> Vec2 {
        Vec2::new(100., 50.)
    }
}

#[derive(Component)]
struct Fireball;

fn shoot(
    mut commands: Commands,
    player_query: Query<(&Player, &Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
) {
    let (_player, transform) = player_query.single();
    if keyboard.just_pressed(KeyCode::Space) {
        // spawn a fireball at player position
        commands
            .spawn(SpriteBundle {
                texture: game_textures.fireball.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 50.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 100.),
                    ..default()
                },
                ..default()
            })
            .insert(Fireball)
            .insert(Projectile::default());
    }
}

fn movement_projectile(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.x -= projectile.speed * time.delta_seconds();

        if transform.translation.x < -800. {
            commands.entity(entity).despawn();
        }
    }
}

fn add_wealth_system(query: Query<(&Enemy, &EnemyState)>, mut player_query: Query<&mut Player>) {
    let mut player = player_query.single_mut();

    for (_enemy, state) in query.iter() {
        match state {
            EnemyState::Death => player.add_wealth(1),
            _ => {}
        }
    }
}

