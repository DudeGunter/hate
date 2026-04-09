use bevy::prelude::*;
use shared::player::{Player, PlayerColor, Position};

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_player);
    app.add_systems(Update, update_player_position);
}

pub fn update_player_position(
    changed_positions: Query<(&mut Transform, &Position), (With<Player>, Changed<Position>)>,
) {
    for (mut transform, position) in changed_positions {
        transform.translation = position.0.xxy().with_z(0.0);
    }
}

pub fn on_add_player(
    trigger: On<Add, Player>,
    mut commands: Commands,
    colors: Query<&PlayerColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = colors.get(trigger.entity).unwrap().0;
    commands.entity(trigger.entity).insert((
        Mesh2d(meshes.add(Rectangle::from_length(10.0))),
        MeshMaterial2d(materials.add(color)),
    ));
}
