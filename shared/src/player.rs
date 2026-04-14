use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.replicate::<Player>()
        .replicate::<PlayerColor>()
        .replicate::<ColorDisplay>();
}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerColor(pub Color);

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorDisplay;

pub mod actions {
    //! player actions.
    //! I think bei may be a very good candidate here
    //! more thoughts...
    use bevy_enhanced_input::prelude::*;

    pub struct MovementContext;

    #[derive(InputAction)]
    #[action_output(bool)]
    pub struct Jump;

    #[derive(InputAction)]
    #[action_output(Vec2)]
    pub struct Move;
}
