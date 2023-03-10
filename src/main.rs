use bevy::prelude::*;

mod commons;
mod player;
mod enemy;
mod collisions;
mod wall;
mod ui;

use commons::setup;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use collisions::CollisionPlugin;
use wall::WallPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                title: "Wizard Defense: The Reckoning".to_string(),
                width: 1600.,
                height: 900.,
                ..Default::default()
            },
            ..Default::default() })
            .set(ImagePlugin::default_nearest()))
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .run();
}