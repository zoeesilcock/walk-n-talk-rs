use bevy::prelude::*;
use components::speech_bubble::{SpeechBubble, SpeechBubbleUI};

pub struct SpeechBubblePlugin;

impl Plugin for SpeechBubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_new_bubbles);
        // app.add_systems(Update, remove_bubbles);
    }
}

#[no_mangle]
fn add_new_bubbles(
    mut commands: Commands,
    query: Query<(Entity, &SpeechBubble, &Transform), Added<SpeechBubble>>,
) {
    for (entity, speech_bubble, transform) in query.iter() {
        spawn_bubble(&mut commands, entity, speech_bubble, transform);
    }
}

// #[no_mangle]
// fn remove_bubbles(
//     mut commands: Commands,
//     mut removals: RemovedComponents<SpeechBubble>,
// ) {
//     for entity in removals.read() {
//         // commands.entity
//     }
// }

#[no_mangle]
fn spawn_bubble(
    commands: &mut Commands,
    owner: Entity,
    speech_bubble: &SpeechBubble,
    transform: &Transform,
) {
    let container = (
        SpeechBubbleUI { owner },
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        },
    );

    let square = (NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(3.)),
            top: Val::Percent(30.),
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.75).into(),
        ..default()
    },);

    let text = TextBundle::from_section(
        &speech_bubble.text,
        TextStyle {
            font_size: 12.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_text_justify(JustifyText::Center);
    println!("Added speech bubble: '{}'", &speech_bubble.text);

    let container_id = commands.spawn(container).id();
    let square_id = commands.spawn(square).id();
    let text_id = commands.spawn(text).id();
    commands.entity(container_id).push_children(&[square_id]);
    commands.entity(square_id).push_children(&[text_id]);
}
