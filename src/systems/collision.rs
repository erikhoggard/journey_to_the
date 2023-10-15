use bevy::prelude::*;

use crate::components::Collider;

pub fn collision(mut colliders: Query<(Entity, &mut Transform, &Collider)>) {
    let mut entities: Vec<_> = colliders
        .iter_mut()
        .map(|(ent, trans, col)| (ent, trans, col.radius))
        .collect();

    for i in 0..entities.len() {
        let pos1 = entities[i].1.translation;
        let rad1 = entities[i].2;

        let mut translation1_change = Vec3::ZERO;

        for j in i + 1..entities.len() {
            let pos2 = entities[j].1.translation;
            let rad2 = entities[j].2;

            // broad-phase aabb check
            if pos1.x - rad1 > pos2.x + rad2
                || pos1.x + rad1 < pos2.x - rad2
                || pos1.y - rad1 > pos2.y + rad2
                || pos1.y + rad1 < pos2.y - rad2
            {
                continue; // not colliding
            }

            // narrow-phase collision check
            let distance = pos1.distance(pos2);
            if distance < rad1 + rad2 {
                let overlap = rad1 + rad2 - distance;
                let direction = (pos2 - pos1).normalize();

                translation1_change -= direction * (overlap * 0.5);

                let trans2 = &mut entities[j].1;
                trans2.translation += direction * (overlap * 0.5);
            }
        }

        let trans1 = &mut entities[i].1;
        trans1.translation += translation1_change;
    }
}
