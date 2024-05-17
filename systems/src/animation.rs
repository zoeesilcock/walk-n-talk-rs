use bevy::prelude::*;
use components::animation::Animated;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

#[no_mangle]
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
