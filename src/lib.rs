//! Contains some stuff :)
//!
//!

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, io, path::PathBuf};

mod widgets {
    pub mod confirm_popup;
    pub mod select_menu;
}

use self::widgets::select_menu::SelectMenu;

/// A trait that provides methods to handle keys. Any widget which takes key input *should* implement this
/// trait.
pub trait KeyHandler {
    /// Executes logic according to the given [`KeyCode`].
    fn handle_key_code(self, key_code: KeyCode);

    // TODO - Says that this window (which is currently focused) has had a movement that brings it out of focus
    // fn surrender_focus(self);
}

/// App instance. Contains logic to draw onto the terminal and to handle key presses.
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    sel_menu: SelectMenu,
}

impl App {
    /// Main event loop. Draws this [`App`] onto the given `&mut DefaultTerminal` and handles events.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Draws this app onto the given `&mut Frame`.
    ///
    /// It draws by rendering itself as a [`Widget`]. See: `impl Widget for &mut App`
    fn draw(&mut self, frame: &mut Frame) {
        if self.sel_menu.is_typing() {
            if let Some((pos_x, pos_y)) = self.sel_menu.get_cursor_ind() {
                frame.set_cursor_position(Position::new((pos_x + 4) as u16, (pos_y + 3) as u16));
            }
        }
        frame.render_widget(self, frame.area());
    }

    /// Blocks and handles the next key event from [`event::read()`].
    ///
    /// Will forward any [`KeyCode`]s to [`KeyHandler::handle_key_code()`] for app logic.
    ///
    /// # Errors
    /// Propagates errors from [`event::read()`].
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                if let KeyCode::Char('q') = event.code {
                    self.exit = true;
                }
                self.handle_key_code(event.code);
            }
            _ => {}
        };
        Ok(())
    }
}

impl KeyHandler for &mut App {
    fn handle_key_code(self, key_code: KeyCode) {
        self.sel_menu.handle_key_code(key_code);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Top ".into(),
            "<Left> / <h>".blue().bold(),
            " | Down ".into(),
            "<Down> / <j>".blue().bold(),
            " | Up ".into(),
            "<Up> / <k>".blue().bold(),
            " | Select/Unselect ".into(),
            "<Right> / <l>".blue().bold(),
            " | Edit ".into(),
            "<Enter>".blue().bold(),
            " | Pick Up/Drop ".into(),
            "<Space>".blue().bold(),
            " | Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered().border_set(border::THICK);

        let [top, main, bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        Paragraph::new(Text::from("Pathui"))
            .centered()
            .block(block)
            .render(top, buf);

        self.sel_menu.render(main, buf);

        instructions.render(bottom, buf);
    }
}

/// Helper method to read the environment `PATH` variable and populate a [`Vec`] with each path, split at `:`'s.
pub fn read_path() -> io::Result<Vec<PathBuf>> {
    let path = env::var_os("PATH").expect("PATH variable is not present!");

    let paths = env::split_paths(&path).collect::<Vec<_>>();

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
