use bevy::prelude::*;
use shared::player::Player;

use crate::control::LocallyOwned;

#[derive(Component, Reflect)]
pub struct PlayerCamera {
    pub entity: Entity,
    pub translation_offset: Vec3,
}

pub fn move_player_camera(
    local_player_query: Query<(&Transform, &PlayerCamera), (With<Player>, With<LocallyOwned>)>,
    mut player_camera_transform: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok((transform, camera_ref)) = local_player_query.single()
        && let Ok(mut camera_transform) = player_camera_transform.get_mut(camera_ref.entity)
    {
        let new_translation = transform.translation + camera_ref.translation_offset;
        camera_transform.translation = new_translation;
    }
}
