use crate::person::Direction;
use crate::player::Player;
use crate::speech_bubble::SpeechBubble;
use bevy::prelude::*;

const MAX_INTERACTION_DISTANCE: f32 = 12.;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_visible_interaction);
        app.add_systems(Update, handle_input);
    }
}

#[derive(Component)]
pub struct Interactable {
    pub interactions: Vec<Interaction>,
}

pub struct Interaction {
    pub max_distance: f32,
    pub interaction_type: InteractionType,
}

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
pub struct CurrentInteraction;

fn update_visible_interaction(
    mut commands: Commands,
    query: Query<(&Interactable, &Transform)>,
    player_query: Query<(&Transform, &Direction), With<Player>>,
    current_interaction_query: Query<Entity, With<CurrentInteraction>>,
) {
    let (player, player_direction) = player_query.single();
    let mut nearby_interactions: Vec<&Interaction> = Vec::new();

    for (interactable, transform) in query.iter() {
        for interaction in &interactable.interactions {
            let distance = (player.translation.x - transform.translation.x).abs();
            let direction = if player.translation.x > transform.translation.x {
                Direction::Left
            } else {
                Direction::Right
            };

            if distance <= interaction.max_distance && player_direction == &direction {
                nearby_interactions.push(interaction);
            }
        }
    }

    // TODO: Sort interactions based on distance and priority.

    // Display the first entry.
    if let Ok(current_interaction) = current_interaction_query.get_single() {
        if nearby_interactions.len() == 0 {
            commands.entity(current_interaction).despawn_recursive();
        }
    } else if nearby_interactions.len() > 0 {
        spawn_text(commands, nearby_interactions[0]);
    }
}

fn spawn_text(mut commands: Commands, interaction: &Interaction) {
    let text_content = match interaction.interaction_type {
        InteractionType::TALK => "Talk",
    };

    let container = (
        CurrentInteraction,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        },
    );

    let square = (NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            bottom: Val::Px(32.),
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.75).into(),
        ..default()
    },);

    let text = TextBundle::from_section(
        text_content,
        TextStyle {
            font_size: 24.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_text_justify(JustifyText::Center);

    let container_id = commands.spawn(container).id();
    let square_id = commands.spawn(square).id();
    let text_id = commands.spawn(text).id();
    commands.entity(container_id).push_children(&[square_id]);
    commands.entity(square_id).push_children(&[text_id]);
}

fn handle_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<&CurrentInteraction>,
    player_query: Query<Entity, (With<Player>, Without<SpeechBubble>)>,
) {
    if let Ok(interaction) = query.get_single() {
        if keys.just_pressed(KeyCode::Space) {
            if let Ok(player) = player_query.get_single() {
                commands.entity(player).insert(SpeechBubble {
                    text: "Hello, World!".to_string(),
                });
            }
        }
    }
}
