use crate::person::PersonBundle;
use bevy::prelude::*;
use components::interaction::Interactable;
use components::movable::Velocity;
use components::npc::Npc;
use components::person::{Direction, PersonAssets};
use rand::{thread_rng, Rng};
use std::time::Duration;

const NPC_COUNT: u32 = 16;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, update);
    }
}

fn setup(mut commands: Commands, person_assets: Res<PersonAssets>) {
    let mut rng = thread_rng();

    for _i in 0..NPC_COUNT {
        let x: f32 = rng.gen_range(-256.0..256.0);
        let direction: Direction = if rng.gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };
        let velocity = if rng.gen_bool(0.3) {
            Velocity {
                x: rng.gen_range(MOVEMENT_SPEED_FROM..MOVEMENT_SPEED_TO),
                y: 0.,
            }
        } else {
            Velocity::default()
        };

        commands.spawn(new_npc(&person_assets, x, direction, velocity));
    }
}

fn new_npc(
    person_assets: &Res<PersonAssets>,
    horizontal_position: f32,
    direction: Direction,
    velocity: Velocity,
) -> (Npc, Interactable, PersonBundle) {
    (
        Npc::default(),
        Interactable::talk(),
        PersonBundle::new(
            person_assets.texture.clone(),
            person_assets.layout.clone(),
            Transform::from_xyz(horizontal_position, 0., 0.),
            direction,
            Some(velocity),
        ),
    )
}

const MOVEMENT_SPEED_FROM: f32 = 20.;
const MOVEMENT_SPEED_TO: f32 = 36.;
const MOVEMENT_TIME_FROM: u64 = 1000;
const MOVEMENT_TIME_TO: u64 = 6000;
const IDLE_TIME_FROM: u64 = 2000;
const IDLE_TIME_TO: u64 = 10000;

#[no_mangle]
fn update(mut query: Query<(&mut Npc, &mut Velocity, &Direction)>, time: Res<Time>) {
    let mut rng = thread_rng();

    for (mut npc, mut velocity, direction) in query.iter_mut() {
        if npc.timer.tick(time.delta()).just_finished() {
            if velocity.x == 0. {
                let random_movement_speed = rng.gen_range(MOVEMENT_SPEED_FROM..MOVEMENT_SPEED_TO);

                if direction == &Direction::Right {
                    velocity.x = -random_movement_speed;
                } else {
                    velocity.x = random_movement_speed;
                }

                npc.timer.set_duration(Duration::from_millis(
                    rng.gen_range(MOVEMENT_TIME_FROM..MOVEMENT_TIME_TO),
                ));
            } else {
                velocity.x = 0.;

                npc.timer.set_duration(Duration::from_millis(
                    rng.gen_range(IDLE_TIME_FROM..IDLE_TIME_TO),
                ));
            }
        }
    }
}
