use bevy::prelude::*;

use crate::person::{PersonAssets, PersonBundle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
}

fn setup(mut commands: Commands, person_assets: Res<PersonAssets>) {
    commands.spawn((
        Player,
        PersonBundle::new(
            person_assets.texture.clone(),
            person_assets.layout.clone(),
            Transform::from_xyz(0., 0., 0.),
        ),
    ));

    commands.spawn(PersonBundle::new(
        person_assets.texture.clone(),
        person_assets.layout.clone(),
        Transform::from_xyz(50., 0., 0.),
    ));
}

#[derive(Component)]
pub struct Player;
