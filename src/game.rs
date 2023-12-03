use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    input::{mouse::{MouseButtonInput, MouseMotion, MouseWheel}, keyboard::KeyboardInput}, 
    window::PrimaryWindow,
    text::{BreakLineOn, Text2dBounds},
};

use crate::{player::PlayerPlugin, health::HealthPlugin, enemy::EnemyPlugin, GameState};

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
        ))
        .add_systems(OnEnter(GameState::GameWon), end_game)
        .add_systems(OnEnter(GameState::GameLost), end_game)
        .add_systems(FixedUpdate, listen_for_restart.run_if(in_state(GameState::GameLost)))
        .add_systems(FixedUpdate, listen_for_restart.run_if(in_state(GameState::GameWon)));
    }
}
#[derive(Component)]
struct EndGameText;

fn end_game(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    game_state: Res<State<GameState>>
 )
    {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text = match **game_state {
        GameState::GameWon => format!("You win! \nPress any key to return to the menu."),
        GameState::GameLost => "Game Over! You lost.\nPress any key to return to the menu.".to_string(),
        _ => "unreachable".to_string()
    };

    let box_color = match **game_state {
        GameState::GameWon => Color::rgb_u8(2, 97, 27),
        GameState::GameLost => Color::rgb_u8(61, 7, 7),
        _ => Color::BLACK
    };

    let box_size = Vec2::new(600.0, 600.0);
    let box_position = Vec2::new(0.0, 0.0);
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: box_color,
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(6.0)),
            ..default()
        }, EndGameText))
        .with_children(|builder| {
            builder.spawn((Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        text,
                        text_style.clone(),
                    )],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z*7.),
                ..default()
            }, EndGameText));
        });
}

fn listen_for_restart(mut commands: Commands, 
    // mut wave_timer: ResMut<WaveTimer>,
    mut key_evr: EventReader<KeyboardInput>, 
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<Entity, With<EndGameText>>) {
    for ev in key_evr.iter() {
        match ev.state {
            _ => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                game_state.set(GameState::Menu);
                // wave_timer.timer.reset();
            }, 
        }
    }
}