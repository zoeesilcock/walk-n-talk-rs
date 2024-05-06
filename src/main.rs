mod animation;
mod person;

use bevy::{prelude::*, window::WindowResolution};

use animation::AnimationPlugin;
use person::PersonBundle;

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
        .add_systems(Startup, setup)
        .run()
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let origin_transform = Transform::from_xyz(0., 18., 0.);
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

    let person_texture = asset_server.load("person.png");
    let person_layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(person_layout);

    commands.spawn((
        Player,
        PersonBundle::new(
            person_texture.clone(),
            texture_atlas_layout.clone(),
            Transform::from_xyz(0., 0., 0.),
        ),
    ));
}
