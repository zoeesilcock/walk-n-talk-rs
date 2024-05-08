use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct Animation {
    pub name: &'static str,
    pub frames: &'static [usize],
    pub timer: Timer,
}

impl Animation {
    pub fn new(name: &'static str, frames: &'static [usize], duration: Duration) -> Self {
        Self {
            name,
            frames,
            timer: Timer::new(duration, TimerMode::Repeating),
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(mut query: Query<(&mut TextureAtlas, &mut Animation)>, time: Res<Time>) {
    for (mut sprite, mut animation) in query.iter_mut() {
        if animation.timer.tick(time.delta()).just_finished() {
            let current_index = animation
                .frames
                .iter()
                .position(|s| *s == sprite.index)
                .unwrap_or(0);

            let next_index = (current_index + animation.timer.times_finished_this_tick() as usize)
                % animation.frames.len();

            sprite.index = animation.frames[next_index];
        }
    }
}
