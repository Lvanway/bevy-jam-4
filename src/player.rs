use std::time::Duration;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::Rng;
use crate::{game::{LEFT_WALL, RIGHT_WALL, TOP_WALL, BOTTOM_WALL}, GameState};

use super::health::Health;

pub const PLAYER_SIZE: f32 = 20.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 10.7);
const PLAYER_INITIAL_HIT_POINTS: u32 = 100;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Game), spawn_player)
        .add_systems(FixedUpdate, move_player.run_if(in_state(GameState::Game)));
    }
}


#[derive(Component)]
pub struct Player;


fn spawn_player(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
        material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
        transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        ..default()
        }, 
        Player,
        Health {hit_points: PLAYER_INITIAL_HIT_POINTS}
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
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