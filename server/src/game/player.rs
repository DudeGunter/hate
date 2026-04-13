use aeronet::io::Session;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use shared::{GameState, management::ownership::Owner, player::Player};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), spawn_players);
    app.add_systems(
        Update,
        (step_1, step_2)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn spawn_players(mut commands: Commands, sessions: Query<Entity, With<Session>>) {
    for entity in sessions {
        let player = commands
            .spawn((Player, CharacterController, Replicated))
            .id();
        commands.entity(entity).add_one_related::<Owner>(player);
    }
}

#[derive(Component, Reflect)]
#[require(
    LockedAxes::ROTATION_LOCKED,
    RigidBody::Kinematic,
    Collider::cylinder(0.5, 1.0)
)]
pub struct CharacterController;

pub fn step_1() {}

pub fn step_2() {}
