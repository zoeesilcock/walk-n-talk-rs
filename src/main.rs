use bevy::{prelude::*, window::WindowResolution};

#[cfg(not(feature = "reload"))]
use systems::animation::AnimationPlugin;
#[cfg(not(feature = "reload"))]
use systems::background::BackgroundPlugin;
#[cfg(not(feature = "reload"))]
use systems::camera::CameraPlugin;
#[cfg(not(feature = "reload"))]
use systems::debug::DebugPlugin;
#[cfg(not(feature = "reload"))]
use systems::interaction::InteractionPlugin;
#[cfg(not(feature = "reload"))]
use systems::movable::MovablePlugin;
#[cfg(not(feature = "reload"))]
use systems::npc::NpcPlugin;
#[cfg(not(feature = "reload"))]
use systems::person::PersonPlugin;
#[cfg(not(feature = "reload"))]
use systems::player::PlayerPlugin;
#[cfg(not(feature = "reload"))]
use systems::speech_bubble::SpeechBubblePlugin;

#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    pub use components::*;
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

    hot_functions_from_file!("systems/src/animation.rs");
    hot_functions_from_file!("systems/src/background.rs");
    hot_functions_from_file!("systems/src/camera.rs");
    hot_functions_from_file!("systems/src/debug.rs");
    hot_functions_from_file!("systems/src/interaction.rs");
    hot_functions_from_file!("systems/src/movable.rs");
    hot_functions_from_file!("systems/src/npc.rs");
    hot_functions_from_file!("systems/src/person.rs");
    hot_functions_from_file!("systems/src/player.rs");
    hot_functions_from_file!("systems/src/speech_bubble.rs");
}

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
