use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Component)]
pub struct Animated {
    pub current_animation: u32,
    pub animations: HashMap<u32, Animation>,
    pub timer: Timer,
}

impl Animated {
    pub fn new(initial_animation: u32, animation_list: Vec<Animation>) -> Self {
        let animations = animation_list.into_iter().map(|a| (a.id, a)).collect();

        Self {
            current_animation: initial_animation,
            animations,
            timer: Timer::new(Duration::ZERO, TimerMode::Repeating),
        }
    }
}

pub struct Animation {
    pub id: u32,
    pub frames: &'static [usize],
    pub frame_rate: u64,
}

impl Animation {
    pub fn new(id: u32, frames: &'static [usize], frame_rate: u64) -> Self {
        Self {
            id,
            frames,
            frame_rate,
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(mut query: Query<(&mut TextureAtlas, &mut Animated)>, time: Res<Time>) {
    for (mut sprite, mut animated) in query.iter_mut() {
        if animated.timer.tick(time.delta()).just_finished() {
            let animation = &animated.animations[&animated.current_animation];
            let current_index = animation
                .frames
                .iter()
                .position(|s| *s == sprite.index)
                .unwrap_or(0);

            let next_index = (current_index + animated.timer.times_finished_this_tick() as usize)
                % animation.frames.len();

            sprite.index = animation.frames[next_index];
        }
    }
}
