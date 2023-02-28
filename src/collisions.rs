use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{commons::{GameTextures, }, enemy::Enemy, player::Projectile};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(projectile_enemy_collisions);
    }
}

fn is_collison(center_a: Vec3, size_a: Vec2, center_b: Vec3, size_b: Vec2,) -> bool {
    let collision = collide(
        center_a, 
        size_a, 
        center_b, 
        size_b,
    );

    collision.is_some()
}

fn projectile_enemy_collisions(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    mut projectile_query: Query<(Entity, &Projectile, &Transform)>,
) {
    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
        for (mut entity, projectile, proj_transform) in projectile_query.iter_mut() {
            let enemy_center = Vec3::new(
                enemy_transform.translation.x,
                enemy_transform.translation.y,
                1.
            );

            let enemy_size = Enemy::get_hurtbox();

            let proj_center = Vec3::new(
                proj_transform.translation.x,
                proj_transform.translation.y,
                1.
            );

            let proj_size = Projectile::get_hitbox();

            if is_collison(enemy_center, enemy_size, proj_center, proj_size) {
                // despawn bullet damage enemy
                commands.entity(entity).despawn();
                // damage enemy
                enemy.apply_damage(projectile.damage());
            }
        }
    } 
}