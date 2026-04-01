//! Relay ratatui/crossterm events into ecs message
use bevy::prelude::*;
use crossterm::event::{self, Event, KeyEvent, MouseEvent};

pub fn register_messages_plugin(app: &mut App) {
    app.add_message::<KeyMessage>();
    app.add_message::<MouseMessage>();
    app.add_message::<TerminalResized>();
    app.add_message::<StringPasted>();
    app.add_message::<Focus>();
}

#[derive(Message, Deref)]
pub struct KeyMessage(KeyEvent);

#[derive(Message, Deref)]
pub struct MouseMessage(MouseEvent);

#[derive(Message)]
pub struct TerminalResized {
    pub x: u16,
    pub y: u16,
}

#[derive(Message, Deref)]
pub struct StringPasted(pub String);

#[derive(Message)]
pub enum Focus {
    Gained,
    Lost,
}

pub fn capture_and_relay_events(
    mut key_messages: MessageWriter<KeyMessage>,
    mut terminal_focus: MessageWriter<Focus>,
    mut mouse_message: MessageWriter<MouseMessage>,
    mut terminal_resized: MessageWriter<TerminalResized>,
    mut string_pasted: MessageWriter<StringPasted>,
) {
    while let Ok(event) = event::read() {
        match event {
            Event::Key(key) => {
                key_messages.write(KeyMessage(key));
            }
            Event::FocusGained => {
                terminal_focus.write(Focus::Gained);
            }
            Event::FocusLost => {
                terminal_focus.write(Focus::Lost);
            }
            Event::Resize(x, y) => {
                terminal_resized.write(TerminalResized { x, y });
            }
            Event::Paste(string) => {
                string_pasted.write(StringPasted(string));
            }
            Event::Mouse(event) => {
                mouse_message.write(MouseMessage(event));
            }
        }
    }
}
