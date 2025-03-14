use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, io, path::PathBuf};

mod widgets {
    pub mod select_menu;
}

use self::widgets::select_menu::SelectMenu;

#[derive(Debug, PartialEq, Eq)]
enum InputMode {
    Typing,
    Navigating,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Navigating
    }
}

/// App instance. Contains logic to draw onto the terminal and to handle key presses.
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    sel_menu: SelectMenu,
    mode: InputMode,
}

impl App {
    /// Main event loop. Draws this `App` onto the given `&mut DefaultTerminal` and handles events.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Draws this app onto the given `&mut Frame`.
    ///
    /// It draws by rendering itself as a `Widget`. See: `impl Widget for &mut App`
    fn draw(&mut self, frame: &mut Frame) {
        if self.mode == InputMode::Typing {
            if let Some((pos_x, pos_y)) = self.sel_menu.get_cursor_ind() {
                frame.set_cursor_position(Position::new((pos_x + 5) as u16, (pos_y + 3) as u16));
            }
        }
        frame.render_widget(self, frame.area());
    }

    /// Blocks and handles the next key event from `event::read()`.
    ///
    /// Will forward any `KeyPress`'s to `handle_key_event()` for app logic.
    ///
    /// # Errors
    /// Propagates errors from `event::read()`.
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_event(event);
            }
            _ => {}
        };
        Ok(())
    }

    /// Handles a given `KeyEvent` by dispatching the correct method accordingly.
    /// In other words, a fancy `match` expression.
    fn handle_key_event(&mut self, event: KeyEvent) {
        match self.mode {
            InputMode::Navigating => self.handle_nav_key_event(event),
            InputMode::Typing => self.handle_type_key_event(event),
        }
    }

    /// Handles a key event in Navigating mode.
    fn handle_nav_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => self.toggle_mode(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Char('k') | KeyCode::Up => self.sel_menu.sel_prev(),
            KeyCode::Char('j') | KeyCode::Down => self.sel_menu.sel_next(),
            KeyCode::Char('h') | KeyCode::Left => self.sel_menu.sel_first(),
            KeyCode::Char('l') | KeyCode::Right => self.sel_menu.toggle_status(),
            _ => {}
        }
    }

    /// Handles a key event in Typing mode.
    fn handle_type_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => self.toggle_mode(),
            KeyCode::Backspace => self.sel_menu.del_char_from_sel_path(),
            KeyCode::Char(c) => self.sel_menu.add_char_to_sel_path(c),
            _ => {}
        }
    }

    /// Helper to toggle the mode between Navigating and Typing.
    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            InputMode::Navigating => InputMode::Typing,
            InputMode::Typing => InputMode::Navigating,
        }
    }
}

impl Widget for &mut App {
    /// TODO: Doc comment
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Pathui".bold());
        // let instructions = Line::from(vec![
        //     " Decrement ".into(),
        //     "<Left>".blue().bold(),
        //     " Increment ".into(),
        //     "<Right>".blue().bold(),
        //     " Quit ".into(),
        //     "<Q> ".blue().bold(),
        // ]);

        let block = Block::bordered()
            //     .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.counter.to_string().yellow(),
        // ])]);

        // Paragraph::new(counter_text)
        // .centered()
        // .block(block)
        // .render(area, buf);
        let [top, main] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(14)])
            .flex(layout::Flex::Legacy)
            .areas(area);

        Paragraph::new(Text::from("Pathui"))
            .centered()
            .block(block)
            .render(top, buf);

        self.sel_menu.render(main, buf);
    }
}

// impl StatefulWidget for &SelectMenu {
//     type State = ListState;

//     /// TODO: Doc comment
//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         // TODO: Clone? Really?
//         let list = List::new(self.items.clone());

//         StatefulWidget::render(list, area, buf, state);
//     }
// }

/// Helper method to read the environment PATH variable and populate a `Vec` with each path, split at `:`'s.
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
