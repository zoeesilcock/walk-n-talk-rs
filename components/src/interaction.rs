use bevy::prelude::*;

const MAX_INTERACTION_DISTANCE: f32 = 12.;

#[derive(Component)]
pub struct Interactable {
    pub interactions: Vec<Interaction>,
}

#[derive(Clone)]
pub struct Interaction {
    pub max_distance: f32,
    pub interaction_type: InteractionType,
}

#[derive(Clone)]
pub enum InteractionType {
    TALK,
}

impl Interactable {
    pub fn talk() -> Self {
        Self {
            interactions: vec![Interaction {
                max_distance: MAX_INTERACTION_DISTANCE,
                interaction_type: InteractionType::TALK,
            }],
        }
    }
}

#[derive(Component)]
pub struct CurrentInteraction {
    pub interaction: Interaction,
}
