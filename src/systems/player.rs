use bevy::prelude::*;

use crate::components::Player;

pub fn player_movement(
    mut q: Query<(&mut Transform, &Sprite), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, _) = q.single_mut();
    if input.pressed(KeyCode::A) {
        transform.translation.x -= 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::W) {
        transform.translation.y += 100.0 * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= 100.0 * time.delta_seconds();
    }
}
