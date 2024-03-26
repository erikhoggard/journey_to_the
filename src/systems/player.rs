use bevy::prelude::*;

use crate::components::Player;

pub fn player_movement(
    mut q: Query<(&mut Transform, &Sprite), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, _) = q.single_mut();
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyW) {
        transform.translation.y += 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.y -= 100.0 * time.delta_seconds();
    }
}
