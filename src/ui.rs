use bevy::{prelude::*, winit::WinitSettings};

use crate::{player::Player, wall::Wall};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_hud)
            .add_startup_system(spawn_upgrade_buttons)
            .add_system(update_hud)
            .insert_resource(WinitSettings::game());
    }
}

#[derive(Component)]
struct Hud;

// Systems for the hud
fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.,
        color: Color::WHITE,
    };

    let hud_text = Text::from_sections([
        TextSection::new("Health: 200 \n", text_style.clone()),
        TextSection::new("$0", text_style.clone()),
    ]);

    commands
        .spawn(Text2dBundle {
            text: hud_text.with_alignment(TextAlignment::TOP_LEFT),
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

fn spawn_upgrade_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let size = Size::new(Val::Px(150.), Val::Px(50.));
    let margin = UiRect::all(Val::Auto);
    let justify_content = JustifyContent::Center;
    let align_items = AlignItems::Center;
    let position = UiRect::new(Val::Px(-700.), Val::Px(450.), Val::Px(-550.), Val::Px(400.));

    let style = Style {
        size,
        margin,
        justify_content,
        align_items,
        ..default()
    };

    commands
        .spawn(ButtonBundle {
            style: style.clone(),
            transform: Transform::from_xyz(-1500., 400., 200.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Repair Wall \n $100",
                TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 20.,
                    color: Color::rgb(0., 0., 0.),
                },
            ));
        });

    commands
        .spawn(ButtonBundle {
            style: style.clone(),
            transform: Transform::from_xyz(-1500., 400., 200.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Upgrade Damage \n $800",
                TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 20.,
                    color: Color::rgb(0., 0., 0.),
                },
            ));
        });

    commands
        .spawn(ButtonBundle {
            style: style.clone(),
            transform: Transform::from_xyz(-1500., 400., 200.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Upgrade Wall Health \n $1000",
                TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 20.,
                    color: Color::rgb(0., 0., 0.),
                },
            ));
        });
}
