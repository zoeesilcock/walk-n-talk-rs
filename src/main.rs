use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run()
}

#[derive(Component)]
struct Background;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Hello, world!");
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.2666;
    commands.spawn(camera);

    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}
