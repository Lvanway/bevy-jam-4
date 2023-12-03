use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::{mouse::{MouseButtonInput, MouseMotion, MouseWheel}, keyboard::KeyboardInput}, 
    window::PrimaryWindow,
    text::{BreakLineOn, Text2dBounds},
};

use crate::{player::PlayerPlugin, health::HealthPlugin, enemy::EnemyPlugin};

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;


pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnemyPlugin,
            HealthPlugin,
            PlayerPlugin
        ));
    }
}