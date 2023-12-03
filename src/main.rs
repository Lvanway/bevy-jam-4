#![allow(clippy::too_many_arguments, clippy::type_complexity)]
use std::time::Duration;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use enemy::EnemyPlugin;
use game::GamePlugin;
use health::HealthPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use rand::Rng;
use splash::SplashPlugin;


mod enemy;
mod health;
mod player;
mod splash;
mod menu;
mod game;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            MenuPlugin,
            SplashPlugin
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
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
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    Pause,
    GameLost,
    GameWon
}

  
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);


const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

  // Generic system that takes a component as a parameter, and will despawn all entities with that component
  fn despawn_with_component<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
  }