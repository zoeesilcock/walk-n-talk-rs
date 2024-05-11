use bevy::prelude::*;
use std::time::Duration;

use crate::animation::{Animated, Animation};
use crate::movable::{Movable, Velocity};

const IDLE_ID: u32 = 1;
const IDLE_FRAMES: &[usize] = &[0];
const IDLE_RATE: u64 = 100;

const WALK_ID: u32 = 2;
const WALK_FRAMES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const WALK_RATE: u64 = 3300;

#[derive(Component)]
pub struct Person;

#[derive(Component, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

#[derive(Bundle)]
pub struct PersonBundle {
    person: Person,
    movable: Movable,
    direction: Direction,
    velocity: Velocity,
    animated: Animated,
    sprite: SpriteSheetBundle,
}

impl PersonBundle {
    pub fn new(
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        transform: Transform,
        direction: Direction,
    ) -> Self {
        Self {
            person: Person,
            movable: Movable,
            direction,
            velocity: Velocity { x: 0., y: 0. },
            animated: Animated::new(
                IDLE_ID,
                vec![
                    Animation::new(IDLE_ID, IDLE_FRAMES, IDLE_RATE),
                    Animation::new(WALK_ID, WALK_FRAMES, WALK_RATE),
                ],
            ),
            sprite: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout,
                    index: IDLE_FRAMES[0],
                },
                transform,
                ..default()
            },
        }
    }
}

#[derive(Resource)]
pub struct PersonAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_direction, update_sprite_direction).chain());
        app.add_systems(Update, update_animation);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_asset_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("person.png");
    let texture_layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 11, 1, None, None);
    let layout = texture_asset_layouts.add(texture_layout);

    commands.insert_resource(PersonAssets { texture, layout });
}

fn update_direction(
    mut query: Query<(&mut Direction, &Velocity), (With<Person>, Changed<Velocity>)>,
) {
    for (mut direction, velocity) in query.iter_mut() {
        if velocity.x > 0. && direction.as_ref() == &Direction::Left {
            *direction = Direction::Right;
        } else if velocity.x < 0. && direction.as_ref() == &Direction::Right {
            *direction = Direction::Left;
        }
    }
}

fn update_sprite_direction(
    mut query: Query<(&Direction, &mut Sprite), (With<Person>, Changed<Direction>)>,
) {
    for (direction, mut sprite) in query.iter_mut() {
        match direction {
            Direction::Right => sprite.flip_x = false,
            Direction::Left => sprite.flip_x = true,
        }
    }
}

fn update_animation(
    mut query: Query<(&Velocity, &mut Animated), (With<Person>, Changed<Velocity>)>,
) {
    for (velocity, mut animated) in query.iter_mut() {
        // Update which animation is playing.
        if velocity.x != 0. && animated.current_animation == IDLE_ID {
            animated.current_animation = WALK_ID;
        } else if velocity.x == 0. && animated.current_animation == WALK_ID {
            animated.current_animation = IDLE_ID;
            animated
                .timer
                .set_duration(Duration::from_millis(IDLE_RATE));
            continue;
        }

        // Update the rate of the walk animation.
        if animated.current_animation == WALK_ID {
            animated.timer.set_duration(Duration::from_millis(
                (WALK_RATE as f32 / velocity.x.abs()) as u64,
            ));
        }
    }
}
