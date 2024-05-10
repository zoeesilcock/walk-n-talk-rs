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
use player::PlayerPlugin;

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
        .run()
}

#[derive(Component)]
struct Background;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let origin_transform = Transform::from_xyz(0., 18., -10.);
    let mut camera = Camera2dBundle::default();
    camera.transform = origin_transform;
    commands.spawn(camera);

    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: origin_transform,
            ..default()
        },
    ));
}
