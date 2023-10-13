use std::sync::Arc;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

mod components;

mod systems {
    pub mod enemy;
    pub mod player;
}

use components::Player;
use systems::{enemy::*, player::*};

const SPRITE_SCALE: Vec3 = Vec3::new(2.0, 2.0, 1.0);
const FIXED_TIMESTEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_enemies)
        .add_systems(FixedUpdate, player_movement)
        .add_systems(FixedUpdate, enemy_ai_system)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // load assets
    let player_texture_handle = asset_server.load("Shroom.png");

    commands.spawn(Camera2dBundle::default());

    //TODO sound

    let player_y = 20.0;

    commands.spawn((
        SpriteBundle {
            texture: player_texture_handle,
            transform: Transform {
                translation: Vec3::new(3.0, player_y, 0.0),
                scale: SPRITE_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        },
        Player,
        // Collider,
    ));
}

// #[derive(Component)]
// struct Collider;
