use crate::tui::{
    logging::{LoggedHistory, TraceMessage},
    terminal::TerminalContext,
};
use bevy::prelude::*;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, List, ListItem, ListState},
};

pub fn render(mut context: ResMut<TerminalContext>, history: Res<LoggedHistory>) {
    let _ = context.draw(|frame| {
        let full_area = frame.area();
        let log_block = Block::bordered()
            .border_set(BorderType::Rounded.to_border_set())
            .title("Log info");
        //let log_output_area = log_block.inner(full_area);

        frame.render_widget(log_block, full_area);
        //render_logged_messages(frame, log_output_area, &history);
    });
}

pub fn render_logged_messages(frame: &mut Frame, area: Rect, history: &Res<LoggedHistory>) {
    let list = List::new(history.iter()).scroll_padding(1);
    let mut state = ListState::default();
    state.select_first();
    frame.render_stateful_widget(list, area, &mut state);
}

impl<'a> Into<ListItem<'a>> for &TraceMessage {
    fn into(self) -> ListItem<'a> {
        ListItem::new(format!("{}", self.message))
    }
}

pub fn _input_box() {}
