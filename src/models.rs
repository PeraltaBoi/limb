use crossterm::{event::Event, terminal};

#[derive(Clone, Copy)]
pub enum Mode {
    Command,
    Insert,
    Visual,
    Normal,
}

#[derive(Clone, Copy)]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
}

pub enum Action {
    MoveTo(Point),
    Clear(terminal::ClearType),
    Write(String),
    // Style control
    SetForegroundColor(crossterm::style::Color),
    SetBackgroundColor(crossterm::style::Color),
    SetAttribute(crossterm::style::Attribute), // Bold, Italic, etc
    ResetStyle,
    // Cursor control
    ShowCursor,
    HideCursor,
    SetCursorStyle(CursorStyle),
    SaveCursorPosition,
    RestoreCursorPosition,
    MoveCursorUp(u16),
    MoveCursorDown(u16),
    MoveCursorLeft(u16),
    MoveCursorRight(u16),
    MoveCursorToNextLine(u16),
    MoveCursorToPreviousLine(u16),
    // Scrolling
    ScrollUp(u16),
    ScrollDown(u16),
    // Region control
    SetScrollRegion(u16, u16), // top, bottom lines
    // Buffer operations
    Flush, // Force buffer flush
}

pub enum CommandResult {
    Execute(String),
    Cancel,
}

pub enum EditorAction {
    ChangeMode(Mode),
    Quit,
    WindowInput(Event),
    Command(CommandResult),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub point: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(point: Point, size: Size) -> Self {
        Self { point, size }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
