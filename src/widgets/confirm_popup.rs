//! Confirmation popup window

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    style::palette::tailwind::{Palette, GRAY, SLATE},
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};

struct ConfirmationMenu {}

impl ConfirmationMenu {
    fn confirm() -> bool {
        todo!()
    }
}

impl Widget for &mut ConfirmationMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
