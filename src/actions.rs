use std::{error::Error, io::Write};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyModifiers},
    queue, style,
};

use crate::{
    models::{Action, CommandAction, EditorAction, Mode},
    window::Window,
};

pub fn handle_action(buf: &mut impl Write, action: Action) -> std::io::Result<()> {
    match action {
        Action::Flush => buf.flush()?,
        Action::SaveCursorPosition => queue!(buf, cursor::SavePosition)?,
        Action::RestoreCursorPosition => queue!(buf, cursor::RestorePosition)?,
        Action::MoveTo(point) => queue!(buf, cursor::MoveTo(point.x, point.y))?,
        Action::Write(text) => write!(buf, "{}", text)?,
        Action::SetAttribute(attr) => queue!(buf, style::SetAttribute(attr))?,
        Action::ResetStyle => queue!(buf, style::ResetColor)?,
        Action::SetCursorStyle(style) => {
            let crossterm_style = match style {
                crate::models::CursorStyle::Block => cursor::SetCursorStyle::SteadyBlock,
                crate::models::CursorStyle::Line => cursor::SetCursorStyle::SteadyBar,
                crate::models::CursorStyle::Underline => cursor::SetCursorStyle::SteadyUnderScore,
            };
            queue!(buf, crossterm_style)?
        }
        // ... handle other actions
        _ => (),
    }
    Ok(())
}

pub fn handle_input(window: &Window, event: Event, mode: Mode) -> EditorAction {
    todo!()
}
