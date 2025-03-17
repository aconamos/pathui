//! Confirmation popup window

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Flex,
    prelude::*,
    style::palette::tailwind::{Palette, GRAY, SLATE},
    symbols::border,
    widgets::{Block, Clear, List, ListItem, ListState, Padding, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use std::io;

use crate::KeyHandler;

#[derive(PartialEq, Eq, Default)]
enum Which {
    #[default]
    Left,
    Right,
}

pub struct ConfirmationMenu<'a> {
    question: &'a str,
    exit: bool,
    which: Which,
}

impl<'a> ConfirmationMenu<'a> {
    pub fn new(question: &'a str) -> Self {
        Self {
            question,
            exit: false,
            which: Which::default(),
        }
    }

    pub fn confirm(&mut self, area: Rect, buf: &mut Buffer) -> io::Result<bool> {
        while !self.exit {
            self.render(area, buf);
            // Icky unwrap!
            // TODO: Change this to figure out what errors are possible and how
            // TODO: to recover from them instead of using unwrap()
            self.handle_events()?;
        }

        Ok(self.which == Which::Left)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_code(event.code);
            }
            _ => {}
        };

        Ok(())
    }

    fn toggle_select(&mut self) {
        if self.which == Which::Left {
            self.which = Which::Right;
        } else {
            self.which = Which::Left;
        }
    }
}

impl KeyHandler for &mut ConfirmationMenu<'_> {
    fn handle_key_code(self, key_code: KeyCode) {
        match key_code {
            KeyCode::Right | KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('l') => {
                self.toggle_select()
            }
            KeyCode::Enter => self.exit = true,
            KeyCode::Char('y') => {
                self.which = Which::Left;
                self.exit = true;
            }
            KeyCode::Char('n') => {
                self.which = Which::Right;
                self.exit = true;
            }
            _ => {}
        }
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
            .padding(Padding::new(0, 0, 1, 1))
            .border_set(border::THICK);

        let inner_area = block.inner(area);

        let [left, right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(inner_area);

        block.render(area, buf);

        Paragraph::new(Text::from("(Y)es").bg(highlight_if(Which::Left, &self.which)))
            .wrap(Wrap { trim: true })
            .centered()
            .render(left, buf);

        Paragraph::new(Text::from("(N)o").bg(highlight_if(Which::Right, &self.which)))
            .wrap(Wrap { trim: true })
            .centered()
            .render(right, buf);
    }
}

fn highlight_if(this_which: Which, that_which: &Which) -> Color {
    if this_which == *that_which {
        Color::Blue
    } else {
        Color::Reset
    }
}
