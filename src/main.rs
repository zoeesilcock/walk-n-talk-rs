use bevy::{prelude::*, window::WindowResolution};

use systems::animation::AnimationPlugin;
use systems::background::BackgroundPlugin;
use systems::camera::CameraPlugin;
use systems::debug::DebugPlugin;
use systems::interaction::InteractionPlugin;
use systems::movable::MovablePlugin;
use systems::npc::NpcPlugin;
use systems::person::PersonPlugin;
use systems::player::PlayerPlugin;
use systems::speech_bubble::SpeechBubblePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(512., 288.)
                            .with_scale_factor_override(2.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(BackgroundPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(MovablePlugin)
        .add_plugins(PersonPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(NpcPlugin)
        .add_plugins(InteractionPlugin)
        .add_plugins(SpeechBubblePlugin)
        .add_plugins(DebugPlugin)
        .run()
}
