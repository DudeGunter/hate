use bevy::prelude::*;
use shared::{AppState, management::GoTo};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, recieve_goto);
}

pub fn recieve_goto(mut goto: MessageReader<GoTo>, mut game_state: ResMut<NextState<AppState>>) {
    for state in goto.read() {
        info!("Going to state {:?}", state.0);
        game_state.set(state.0);
    }
}
