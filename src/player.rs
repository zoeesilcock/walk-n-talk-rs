use bevy::prelude::*;

use crate::movable::Velocity;
use crate::person::{Direction, PersonAssets, PersonBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
        app.add_systems(Update, handle_input);
    }
}

fn setup(mut commands: Commands, person_assets: Res<PersonAssets>) {
    commands.spawn((
        Player,
        PersonBundle::new(
            person_assets.texture.clone(),
            person_assets.layout.clone(),
            Transform::from_xyz(0., 0., 0.),
            Direction::Right,
        ),
    ));
}

const MOVEMENT_SPEED: f32 = 36.;

#[derive(Component)]
pub struct Player;

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();
    let right_pressed = keys.pressed(KeyCode::ArrowRight);
    let left_pressed = keys.pressed(KeyCode::ArrowLeft);

    if keys.just_pressed(KeyCode::ArrowRight)
        || (keys.just_released(KeyCode::ArrowLeft) && right_pressed)
    {
        // Move right.
        velocity.x = MOVEMENT_SPEED;
    } else if keys.just_pressed(KeyCode::ArrowLeft)
        || (keys.just_released(KeyCode::ArrowRight) && left_pressed)
    {
        // Move left.
        velocity.x = -MOVEMENT_SPEED;
    } else if velocity.x != 0. && !right_pressed && !left_pressed {
        // Reset velocity.
        velocity.x = 0.;
    }
}
