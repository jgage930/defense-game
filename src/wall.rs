use bevy::prelude::*;
use std::default::Default;

use crate::commons::GameTextures;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_wall);
    }
}

#[derive(Resource)]
pub struct Wall {
    health: f32,
    max_health: f32,
}

impl Wall {
    pub const LEFT: f32 = 600.;
    pub const SIZE: f32 = 64.;

    pub fn apply_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn max_health(&self) -> f32 {
        self.max_health
    }

    pub fn repair(&mut self) {
        let repair_amount: f32 = 10.;

        if self.health() + repair_amount > self.max_health() {
            self.health = self.max_health;
        } else {
            self.health += repair_amount;
        }
    }

    pub fn upgrade_max_health(&mut self) {
        let upgrade_amount = 20.;

        self.max_health += upgrade_amount;
        self.health = self.max_health;
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            health: 200.,
            max_health: 200.,
        }
    }
}

fn spawn_wall(mut commands: Commands, game_textures: Res<GameTextures>) {
    commands.insert_resource(Wall::default());

    for y in (-500..500).step_by(64) {
        commands.spawn(SpriteBundle {
            texture: game_textures.wall.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(64.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(Wall::LEFT, y as f32, 10.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

