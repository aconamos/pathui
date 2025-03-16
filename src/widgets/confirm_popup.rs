//! Confirmation popup window

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    style::palette::tailwind::{Palette, GRAY, SLATE},
    symbols::border,
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::KeyHandler;

pub struct ConfirmationMenu<'a> {
    question: &'a str,
}

impl<'a> ConfirmationMenu<'a> {
    pub fn new(question: &'a str) -> Self {
        Self { question }
    }
}

impl KeyHandler for &mut ConfirmationMenu<'_> {
    fn handle_key_code(self, key_code: KeyCode) {
        todo!()
    }
}

impl Widget for &mut ConfirmationMenu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);

        let block = Block::bordered()
            .title(self.question)
            .title_alignment(Alignment::Center)
            .border_set(border::THICK);

        let inner_area = block.inner(area);

        let [left, right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(inner_area);

        block.render(area, buf);

        Paragraph::new("Lorem ipsum")
            .wrap(Wrap { trim: true })
            .render(left, buf);

        Paragraph::new("dolor sit amet")
            .wrap(Wrap { trim: true })
            .render(right, buf);
    }
}

// trait Confirmable??
// This would essentially make every widget that implements it check for whether or not it is in the
// menu before advancing its state. This might work?
