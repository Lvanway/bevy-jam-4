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
use crate::{game::{LEFT_WALL, RIGHT_WALL, TOP_WALL, BOTTOM_WALL}, GameState, player, health::Health};
use super::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WaveTimer {
            timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_INTERVAL_SECONDS as u64), TimerMode::Repeating),
        })
        .add_systems(FixedUpdate, (
            move_enemy, 
            spawn_enemy,
            enemy_damage_player
        ).chain().run_if(in_state(GameState::Game)));
    }
}

const ENEMY_SIZE: f32 = 10.0;
const ENEMY_SPEED: f32 = 0.5;
const ENEMY_COLOR: Color = Color::rgb(10.7, 0.3, 0.3);
const ENEMY_SPAWN_PER_INTERVAL: u32 = 25;
const ENEMY_MIN_SPAWN_DISTANCE: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL_SECONDS: u32 = 1;
pub const ENEMY_DAMAGE: u32 = 10;


#[derive(Component)]
struct Enemy;


#[derive(Resource)]
pub struct WaveTimer {
    pub timer: Timer
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


fn enemy_damage_player(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(&Transform, &mut Health, With<Player>)>,
        Query<(Entity, &Transform, With<Enemy>)>)>) {
        let player_position = query.p0().single().0.translation.clone();

        let mut player_hits = 0;
        for (enemy_entity, enemy_transform, _) in query.p1().iter_mut() {
            let distance = enemy_transform.translation.distance(player_position);
            if distance < player::PLAYER_SIZE {
                commands.entity(enemy_entity).despawn();
                player_hits += 1;
            }
        }
        query.p0().single_mut().1.hit_points -= player_hits * ENEMY_DAMAGE;
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
                        Enemy,
                        Health { hit_points: 100}
                    ));
                }
            }
        }
}

