use bevy::prelude::*;

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}
