use bevy::prelude::*; 

use crate::{wall::Wall, player::Player};


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_hud)
            .add_system(update_hud);
    }
}

#[derive(Component)]
struct Hud;

// Systems for the hud
fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.,
        color: Color::WHITE
    };

    let hud_text =  Text::from_sections([
        TextSection::new("Health: 200 \n", text_style.clone()),
        TextSection::new("$0", text_style.clone()),
    ]);

    commands.spawn(Text2dBundle{
        text: hud_text 
            .with_alignment(TextAlignment::TOP_LEFT),
        transform: Transform {
            translation: Vec3::new(-750., 450., 101.),
            ..Default::default() 
        },
        ..Default::default()
    })
    .insert(Hud);
}

fn update_hud(
    mut hud_query: Query<&mut Text, With<Hud>>,
    player_query: Query<&Player>,
    wall: Res<Wall>,
) {
    let player = player_query.single();

    let mut text = hud_query.single_mut();
    text.sections[0].value = format!("Health: {} \n", wall.health());
    text.sections[1].value = format!("${} \n", player.wealth());
}