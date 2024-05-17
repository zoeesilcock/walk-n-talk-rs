use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Npc {
    pub timer: Timer,
}

impl Npc {
    pub fn default() -> Self {
        Self {
            timer: Timer::new(Duration::ZERO, TimerMode::Repeating),
        }
    }
}
