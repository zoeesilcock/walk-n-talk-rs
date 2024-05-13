mod animation;
mod debug;
mod movable;
mod npc;
mod person;
mod player;

use bevy::{prelude::*, window::WindowResolution};

use animation::AnimationPlugin;
use debug::DebugPlugin;
use movable::MovablePlugin;
use npc::NpcPlugin;
use person::PersonPlugin;
use player::{Player, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(512., 288.)
                            .with_scale_factor_override(2.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AnimationPlugin)
        .add_plugins(MovablePlugin)
        .add_plugins(PersonPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(NpcPlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, update_camera_position)
        .run()
}

const BACKROUND_SCALE: f32 = 4.;

#[derive(Component)]
struct Background;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let origin_transform = Transform::from_xyz(0., 18., -10.);
    let mut camera = Camera2dBundle::default();
    camera.transform = origin_transform;
    commands.spawn(camera);

    commands.spawn((
        Background,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1. / BACKROUND_SCALE,
        },
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: origin_transform.with_scale(Vec3 {
                x: BACKROUND_SCALE,
                y: 1.,
                z: 1.,
            }),
            ..default()
        },
    ));
}

fn update_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
}
