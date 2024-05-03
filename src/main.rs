use bevy::{prelude::*, window::WindowResolution};

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
        .add_systems(Update, animate_sprite)
        .add_systems(Startup, setup)
        .run()
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let origin_transform = Transform::from_xyz(0., 18., 0.);
    let mut camera = Camera2dBundle::default();
    camera.transform = origin_transform;
    commands.spawn(camera);

    commands.spawn((
        Background,
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: origin_transform,
            ..default()
        },
    ));

    let person_texture = asset_server.load("person.png");
    let person_layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 11, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(person_layout);
    let idle_indices = AnimationIndices { first: 0, last: 0 };
    let walk_indices = AnimationIndices { first: 1, last: 10 };

    commands.spawn((
        Player,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        SpriteSheetBundle {
            texture: person_texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: walk_indices.first,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        walk_indices,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
