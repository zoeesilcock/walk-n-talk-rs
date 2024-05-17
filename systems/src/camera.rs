use crate::background::WORLD_ORIGIN;
use bevy::prelude::*;
use components::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera_position);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform = WORLD_ORIGIN;
    commands.spawn(camera);
}

#[no_mangle]
fn update_camera_position(
    mut query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
}
