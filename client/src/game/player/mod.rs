use crate::{control::LocallyOwned, game::player::camera::PlayerCamera};
use avian3d::prelude::TransformInterpolation;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::{
    GameState,
    player::{Player, PlayerColor},
};

mod camera;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_player);
    app.add_systems(OnEnter(GameState::Playing), insert_player_camera);
    app.add_systems(
        Update,
        camera::move_player_camera.run_if(in_state(GameState::Playing)),
    );
}

pub fn insert_player_camera(
    mut commands: Commands,
    local_player_query: Query<Entity, (With<Player>, With<LocallyOwned>)>,
) {
    if let Ok(local_player) = local_player_query.single() {
        let camera_entity = commands
            .spawn((Camera3d::default(), PrimaryEguiContext))
            .id();

        commands.entity(local_player).insert(PlayerCamera {
            entity: camera_entity,
            translation_offset: Vec3::new(0.0, 0.5, 0.0),
        });
    } else {
        info!(
            "Failed to retreive single local player. There are {} local players.",
            local_player_query.count()
        );
    }
}

pub fn on_add_player(
    trigger: On<Add, Player>,
    mut commands: Commands,
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
}
