use bevy::prelude::*;

#[derive(Component)]
pub struct SpeechBubble {
    pub text: String,
}

#[derive(Component)]
pub struct SpeechBubbleUI {
    pub owner: Entity,
}
