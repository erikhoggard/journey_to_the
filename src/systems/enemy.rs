use bevy::prelude::*;

use crate::components::Enemy;
use crate::components::Player;

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("Planet.png"),
            transform: Transform {
                translation: Vec3::new(3.0, 1.0, 0.0),
                // scale: SPRITE_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            ..default()
        })
        .insert(Enemy);
}

pub fn enemy_ai_system(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let player_transform: &Transform = player_query.single();
    let player_pos = player_transform.translation;

    for mut enemy_transform in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation;

        let direction = player_pos - enemy_pos;

        let ndirection = direction.normalize();

        let speed = 33.0;
        if direction.length() > speed {
            enemy_transform.translation += ndirection * speed * time.delta_seconds();
        }
    }
}
