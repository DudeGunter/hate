use aeronet::io::Session;
use avian3d::prelude::*;
use bevy::{prelude::*, time::Stopwatch};
use bevy_replicon::prelude::*;
use shared::{GameState, management::ownership::Owner, player::Player};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), spawn_players);
    app.add_systems(
        Update,
        (step_1, step_2, step_3, step_4, step_5)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn spawn_players(mut commands: Commands, sessions: Query<Entity, With<Session>>) {
    for entity in sessions {
        let player = commands
            .spawn((
                Player,
                CharacterController {
                    speed: 20.0,
                    jump_force: 10.0,
                },
                Replicated,
            ))
            .id();
        commands.entity(entity).add_one_related::<Owner>(player);
    }
}

// Theory from previouse project:
// updated_grounded,
// apply_gravity,
// handle_movement,
// handle_jump,
// apply_forces,
// move_and_slide,
// force_clear_conditional,
// update_camera_transform,
#[derive(Component, Reflect, Debug)]
#[require(
    LockedAxes::ROTATION_LOCKED,
    RigidBody::Kinematic,
    Collider::cylinder(0.5, 1.0),
    AccumulatedInput,
    Gravity(Vec3::new(0.0, -20.0, 0.0)),
    Velocity,
    Forces,
)]
pub struct CharacterController {
    pub speed: f32,
    pub jump_force: f32,
}

#[derive(Component, Default, Debug)]
pub struct AccumulatedInput {
    pub last_movement: Option<Vec2>,
    pub jumped: Option<Stopwatch>,
}

#[derive(Component, Debug)]
pub struct Gravity(pub Vec3);

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Debug)]
pub struct Forces {
    vectors: Vec<Vec3>,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

pub fn capture_actions() {}

pub fn step_1() {}

pub fn step_2() {}

pub fn step_3() {}

pub fn step_4() {}

pub fn step_5() {}
