use bevy::prelude::*;
use std::time::Duration;

use crate::animation::Animation;
use crate::movable::{Movable, Velocity};

const IDLE_FRAMES: &[usize] = &[0];
const WALK_FRAMES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const ANIMATION_FRAME_DURATION: Duration = Duration::from_millis(100);

const IDLE_ANIMATION_NAME: &str = "IDLING";
const IDLE_ANIMATION_RATE: u64 = 100;
const WALK_ANIMATION_NAME: &str = "WALKING";
const WALK_ANIMATION_RATE: f32 = 3300.;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
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
    animation: Animation,
    sprite: SpriteSheetBundle,
}

impl PersonBundle {
    pub fn new(
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        transform: Transform,
    ) -> Self {
        Self {
            person: Person,
            movable: Movable,
            direction: Direction::Right,
            velocity: Velocity { x: 0., y: 0. },
            animation: Animation::new(IDLE_ANIMATION_NAME, IDLE_FRAMES, ANIMATION_FRAME_DURATION),
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
        app.add_systems(Update, update_direction);
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
    mut query: Query<(&Velocity, &mut Animation), (With<Person>, Changed<Velocity>)>,
) {
    for (velocity, mut animation) in query.iter_mut() {
        // Update which animation is playing.
        if velocity.x != 0. && animation.name == IDLE_ANIMATION_NAME {
            animation.name = WALK_ANIMATION_NAME;
            animation.frames = WALK_FRAMES;
        } else if velocity.x == 0. && animation.name == WALK_ANIMATION_NAME {
            animation.name = IDLE_ANIMATION_NAME;
            animation.frames = IDLE_FRAMES;
            animation
                .timer
                .set_duration(Duration::from_millis(IDLE_ANIMATION_RATE));
            return;
        }

        // Update the rate of the walk animation.
        if animation.name == WALK_ANIMATION_NAME {
            animation.timer.set_duration(Duration::from_millis(
                (WALK_ANIMATION_RATE / velocity.x.abs()) as u64,
            ));
        }
    }
}
