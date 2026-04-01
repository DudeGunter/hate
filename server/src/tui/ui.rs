use crate::tui::terminal::TerminalContext;
use bevy::prelude::*;

pub fn render(mut context: ResMut<TerminalContext>) {
    context.draw(|frame| {});
}

pub fn log_box() {}

pub fn input_box() {}
