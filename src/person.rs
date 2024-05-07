use bevy::prelude::*;
use std::time::Duration;

use crate::animation::Animation;
use crate::movable::{Movable, Velocity};

const IDLE_FRAMES: &[usize] = &[0];
const WALKING_FRAMES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const ANIMATION_FRAME_DURATION: Duration = Duration::from_millis(100);

#[derive(Resource)]
pub struct PersonAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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

#[derive(Component)]
pub struct Person;

#[derive(Bundle)]
pub struct PersonBundle {
    person: Person,
    movable: Movable,
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
            velocity: Velocity { x: 0., y: 0. },
            animation: Animation::new(IDLE_FRAMES, ANIMATION_FRAME_DURATION),
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
