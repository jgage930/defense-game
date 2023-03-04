use bevy::{prelude::*, winit::WinitSettings};

use crate::{player::Player, wall::Wall};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_hud)
            .add_startup_system(spawn_upgrade_buttons)
            .add_system(update_hud)
            .add_system(click_button_system)
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

#[derive(Component)]
enum ButtonType {
    RepairWall,
    UpgradeWall,
}

fn spawn_upgrade_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("font.ttf");

    let button_text = [
        "Repair Wall $100",
        "Upgrade Damage $800",
        "Upgrade Wall $1000",
    ];

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|commands| {
            for i in 0..2 {
                let text = button_text[i];
                let button_type = match i {
                    0 => ButtonType::RepairWall,
                    1 => ButtonType::UpgradeWall,
                    _ => ButtonType::RepairWall,
                };

                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(190.), Val::Px(30.)),
                            align_self: AlignSelf::FlexStart,
                            margin: UiRect::all(Val::Percent(2.)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.,
                                color: Color::BLACK,
                            },
                        ));
                    })
                    .insert(button_type);
            }
        });
}

fn click_button_system(
    query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
    mut player_query: Query<&mut Player>,
    mut wall: ResMut<Wall>,
) {
    let mut player = player_query.single_mut();

    for (interaction, button_type) in query.iter() {
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    ButtonType::RepairWall => {
                        let cost: usize = 100;
                        if player.wealth() >= cost {
                            player.spend(cost);
                            wall.repair();
                        }
                    }
                    ButtonType::UpgradeWall => {
                        let cost: usize = 1000;
                        if player.wealth() >= cost {
                            player.spend(cost);
                            wall.upgrade_max_health();
                        }
                    }
                };
            }
            _ => {}
        };
    }
}
