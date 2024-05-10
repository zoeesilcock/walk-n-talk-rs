use crate::person::{Direction, PersonAssets, PersonBundle};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const NPC_COUNT: u32 = 16;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
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

        commands.spawn(new_npc(&person_assets, x, direction));
    }
}

fn new_npc(
    person_assets: &Res<PersonAssets>,
    horizontal_position: f32,
    direction: Direction,
) -> (Npc, PersonBundle) {
    (
        Npc,
        PersonBundle::new(
            person_assets.texture.clone(),
            person_assets.layout.clone(),
            Transform::from_xyz(horizontal_position, 0., 0.),
            direction,
        ),
    )
}

#[derive(Component)]
pub struct Npc;
