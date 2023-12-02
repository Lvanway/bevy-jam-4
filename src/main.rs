// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PLAYER_SIZE: f32 = 20.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 10.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_player)
        .run();
}

fn setup(
    mut commands: Commands,     
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
        }, 
        Player
    ));
}

#[derive(Component)]
struct Player;

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction_y += 1.0;
    }

    // Calculate the new position based on player input
    let new_player_position_x =
        player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    let new_player_position_y =
        player_transform.translation.y + direction_y * PLAYER_SPEED * time.delta_seconds();

    // // Update the player position,
    // // making sure it doesn't cause the player to leave the arena
    let left_bound = LEFT_WALL + PLAYER_SIZE / 2.0;
    let right_bound = RIGHT_WALL - PLAYER_SIZE / 2.0;
    let top_bound = TOP_WALL - PLAYER_SIZE / 2.0;
    let bottom_bound = BOTTOM_WALL + PLAYER_SIZE / 2.0;

    player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_y.clamp(bottom_bound, top_bound);

}
