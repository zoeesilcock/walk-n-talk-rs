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

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement);
    }
}

fn apply_movement(mut query: Query<(&Velocity, &mut Transform), With<Movable>>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * time.delta_seconds();
        translation.y += velocity.y * time.delta_seconds();
    }
}
