#![allow(unused)]

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use components::{Enemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// Asset constants
const PLAYER_SPRITE: &str = "player_a.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

// Game constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
}

// Main App
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Invaders".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // Add WinSize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            // Despawn when out of screen
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    // Iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // Iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // Determine if collision has occured
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // Perform collision
            if let Some(_) = collision {
                // Despawn the enemy
                commands.entity(enemy_entity).despawn();

                // Despawn the laser
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
