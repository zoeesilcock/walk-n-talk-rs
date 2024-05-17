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
