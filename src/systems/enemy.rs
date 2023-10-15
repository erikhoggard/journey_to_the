use bevy::prelude::*;
use rand::Rng;

use crate::components::Collider;
use crate::components::Enemy;
use crate::components::Player;

pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let num_enemies = rand::thread_rng().gen_range(4..=7);

    for _ in 0..num_enemies {
        let x = rand::thread_rng().gen_range(-500.0..=500.0);
        let y = rand::thread_rng().gen_range(-500.0..=500.0);

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("Planet.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    // scale: SPRITE_SCALE,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            })
            .insert(Enemy)
            .insert(Collider { radius: 16.0 });
    }
}

pub fn enemy_ai_system(
    time: Res<Time>,
    player_query: Query<(&Transform, &Collider), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &Collider), (With<Enemy>, Without<Player>)>,
) {
    let (player_transform, player_collider): (&Transform, &Collider) = player_query.single();
    let player_pos = player_transform.translation;
    let player_radius = player_collider.radius;

    for (mut enemy_transform, enemy_collider) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation;
        let enemy_radius = enemy_collider.radius;

        let direction = player_pos - enemy_pos;

        if direction.length() < (player_radius + enemy_radius) + 1.0 {
            continue; //don't move if already colliding with player
        }

        let ndirection = direction.normalize();
        let speed = 33.0;
        enemy_transform.translation += ndirection * speed * time.delta_seconds();
    }
}
