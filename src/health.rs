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
use super::player::Player;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(FixedUpdate, check_player_loss.run_if(in_state(GameState::Game)));
    }
}



#[derive(Component)]
pub struct Health {
    pub hit_points: u32
}



fn check_player_loss(query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if query.single().hit_points <= 0 {
        game_state.set(GameState::GameLost);
    }
}