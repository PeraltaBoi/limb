use crossterm::event::Event;

use crate::models::{Action, Mode};

#[derive(Clone)]
pub enum Widget {
    // TextBuffer(TextBuffer),
}

impl Widget {
    pub fn handle_input(&self, event: Event, mode: Mode) -> Widget {
        match self {
            // Widget::TextBuffer(tb) => Widget::TextBuffer(tb.handle_input(event, mode)),
            // etc
            _ => {
                todo!()
            }
        }
    }

    pub fn render(&self, mode: Mode) -> Vec<Action> {
        match self {
            // Widget::TextBuffer(tb) => tb.render(mode),
            _ => {
                todo!()
            }
        }
    }
}
