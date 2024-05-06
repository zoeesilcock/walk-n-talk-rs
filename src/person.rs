use bevy::prelude::*;
use std::time::Duration;

use crate::animation::Animation;

const IDLE_FRAMES: &[usize] = &[0];
const WALKING_FRAMES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const ANIMATION_FRAME_DURATION: Duration = Duration::from_millis(100);

#[derive(Component)]
pub struct Person;

#[derive(Bundle)]
pub struct PersonBundle {
    person: Person,
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
