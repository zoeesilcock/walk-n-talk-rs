use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

#[derive(Resource)]
pub struct PersonAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
