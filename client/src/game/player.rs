use avian3d::prelude::TransformInterpolation;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::{
    management::ownership::Owns,
    player::{Player, PlayerColor},
};

use crate::control::LocalClient;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_player);
}

pub fn on_add_player(
    trigger: On<Add, Player>,
    mut commands: Commands,
    locally_owned: Single<&Owns, With<LocalClient>>,
    colors: Query<&PlayerColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = colors.get(trigger.entity).unwrap().0;
    commands.entity(trigger.entity).insert((
        TransformInterpolation,
        Mesh3d(meshes.add(Cylinder::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(color))),
    ));

    if locally_owned.contains(&trigger.entity) {
        commands
            .entity(trigger.entity)
            .insert((Camera3d::default(), PrimaryEguiContext));
    }
}
