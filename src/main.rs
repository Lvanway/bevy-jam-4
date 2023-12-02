// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
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
use rand::Rng;


const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const PLAYER_SIZE: f32 = 20.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 10.7);

const ENEMY_SIZE: f32 = 10.0;
const ENEMY_SPEED: f32 = 0.5;
const ENEMY_COLOR: Color = Color::rgb(10.7, 0.3, 0.3);
const ENEMY_SPAWN_PER_INTERVAL: u32 = 25;
const ENEMY_MIN_SPAWN_DISTANCE: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL_SECONDS: u32 = 1;

fn main() {
    App::new()
        .insert_resource(WaveTimer {
            // create the repeating timer
            timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_INTERVAL_SECONDS as u64), TimerMode::Repeating),
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, 
            (move_player,
            move_enemy,
            spawn_enemy).chain())
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

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
pub struct WaveTimer {
    pub timer: Timer
}

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

fn move_enemy(
    time: Res<Time>, 
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Enemy>>)>) {
    
        
        let player_position= query.p0().single().translation.clone();
        let distance_moved = ENEMY_SPEED * time.delta_seconds();

        for mut enemy_transform in query.p1().iter_mut() {
            enemy_transform.translation.x +=
                    distance_moved * (player_position.x - enemy_transform.translation.x);
            enemy_transform.translation.y +=
                    distance_moved * (player_position.y - enemy_transform.translation.y);
        }
}

fn spawn_enemy(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<Enemy>>)>
    ) {
        wave_timer.timer.tick(time.delta());

        let player_position= query.p0().single().translation.clone();

        if wave_timer.timer.finished() {
            let mut rng = rand::thread_rng();

            for _ in 0..ENEMY_SPAWN_PER_INTERVAL {
                let x = rng.gen_range(LEFT_WALL..RIGHT_WALL);
                let y = rng.gen_range(BOTTOM_WALL..TOP_WALL);
                let distance = Vec2::new(x, y).distance(player_position.truncate());
                if distance > ENEMY_MIN_SPAWN_DISTANCE {
                    commands.spawn((MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(ENEMY_SIZE).into()).into(),
                        material: materials.add(ColorMaterial::from(ENEMY_COLOR)),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                        }, 
                        Enemy
                    ));
                }
            }
        }
}

