use bevy::prelude::*;
use components::background::Background;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

const BACKROUND_SCALE: f32 = 4.;
pub const WORLD_ORIGIN: Transform = Transform::from_xyz(0., 18., -10.);

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Background,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1. / BACKROUND_SCALE,
        },
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: WORLD_ORIGIN.with_scale(Vec3 {
                x: BACKROUND_SCALE,
                y: 1.,
                z: 1.,
            }),
            ..default()
        },
    ));
}
