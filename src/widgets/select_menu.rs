//! The [`SelectMenu`] [`Widget`]. Provides a widget to select, move, and edit parts of the PATH variable.
//!
//! TODO: Allow to load other paths
//! TODO: Write paths out

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    style::palette::tailwind::{Palette, GRAY, SLATE},
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, ffi::OsStr, io, path::PathBuf};

use crate::KeyHandler;

/// Widget containing a list of selectable items and a corresponding state.
///
/// To achieve stateful behavior, a [`List`] is created each render and rendered statefully using `self.state`.
#[derive(Debug)]
pub struct SelectMenu {
    items: Vec<PathVar>,
    state: ListState,
    input_mode: InputMode,
}

#[derive(Debug, PartialEq, Eq, Default)]
enum InputMode {
    #[default]
    Navigating,
    Grabbing,
    Typing,
}

/// Represents a path, either on or off.
#[derive(Debug)]
struct PathVar {
    path: PathBuf,
    active: bool,
}

impl PathVar {
    /// Constructs a new [`PathVar`] with the given path and set to true.
    fn new(path: PathBuf) -> Self {
        PathVar { path, active: true }
    }
}

impl AsRef<OsStr> for PathVar {
    fn as_ref(&self) -> &OsStr {
        self.path.as_os_str()
    }
}

impl SelectMenu {
    /// Creates a new [`SelectMenu`] with the contents of the environment `PATH` variable and a default [`ListState`].
    pub fn new() -> SelectMenu {
        let mut state = ListState::default();
        state.select(Some(0));

        SelectMenu {
            items: crate::read_path()
                .unwrap()
                .into_iter()
                .map(|path| PathVar::new(path))
                .collect(),
            state,
            input_mode: InputMode::Navigating,
        }
    }

    /// Gets the x-value corresponding the position of the cursor
    pub fn get_cursor_ind(&mut self) -> Option<(usize, usize)> {
        if let Some(i) = self.state.selected() {
            let path_var = &mut self.items[i];

            let os_str = path_var.path.as_mut_os_str().to_owned();

            return Some((os_str.len(), i));
        }

        None
    }

    fn sel_next(&mut self) {
        self.state.select_next();
    }

    fn sel_prev(&mut self) {
        self.state.select_previous();
    }

    fn sel_first(&mut self) {
        self.state.select_first();
    }

    fn sel_last(&mut self) {
        self.state.select_last();
    }

    /// Toggle the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].active = !self.items[i].active
        }
    }

    /// Writes the given path contained within `self.items` to the environment.
    fn write_path_to_env(&self) -> io::Result<()> {
        let paths: Vec<&OsStr> = self
            .items
            .iter()
            .filter(|path_var| path_var.active)
            .map(|path_var| path_var.as_ref())
            .collect();

        env::set_var(
            "PATH",
            env::join_paths(paths).expect("Failure joining paths!"),
        );

        Ok(())
    }

    /// Appends to the selected path.
    fn add_char_to_sel_path(&mut self, c: char) {
        if let Some(i) = self.state.selected() {
            let path_var = &mut self.items[i];

            // Convert PathVar into a string, push to string, put back
            let mut os_str = path_var.path.as_mut_os_str().to_owned();
            os_str.push(c.to_string());
            if os_str.len() == 1 {
                path_var.active = true;
            }
            self.items[i].path = os_str.into();
        }
    }

    /// Removes character from the selected path.
    fn del_char_from_sel_path(&mut self) {
        if let Some(i) = self.state.selected() {
            let path_var = &mut self.items[i];

            // Convert PathVar into a string, push to string, put back
            let os_str = path_var.path.to_str().unwrap();
            if os_str.is_empty() {
                return;
            }
            let (os_str, _) = os_str.split_at(os_str.len() - 1);
            if os_str.is_empty() {
                path_var.active = false;
            }
            self.items[i].path = os_str.into();
        }
    }

    /// Swaps the current item with the one above and keeps the selection, if possible
    fn swap_up(&mut self) {
        if let Some(i) = self.state.selected() {
            if i == 0 {
                return;
            }

            self.items.swap(i, i - 1);

            self.sel_prev();
        }
    }

    /// Swaps the current item with the one below and keeps the selection, if possible
    fn swap_down(&mut self) {
        if let Some(i) = self.state.selected() {
            if i == self.items.len() - 1 {
                return;
            }

            self.items.swap(i, i + 1);

            self.sel_next();
        }
    }

    /// Handles a key event in Navigating mode.
    fn handle_nav_key_code(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => self.input_mode = InputMode::Typing,
            KeyCode::Char(' ') => self.input_mode = InputMode::Grabbing,
            // KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Char('k') | KeyCode::Up => self.sel_prev(),
            KeyCode::Char('j') | KeyCode::Down => self.sel_next(),
            KeyCode::Char('h') | KeyCode::Left => self.sel_first(),
            KeyCode::Char('l') | KeyCode::Right => self.toggle_status(),
            _ => {}
        }
    }

    /// Handles a key event in Typing mode.
    fn handle_type_key_code(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter => self.input_mode = InputMode::Navigating,
            KeyCode::Backspace => self.del_char_from_sel_path(),
            KeyCode::Char(c) => self.add_char_to_sel_path(c),
            _ => {}
        }
    }

    /// Handles a key event in Grabbing mode.
    fn handle_grab_key_code(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(' ') => self.input_mode = InputMode::Navigating,
            KeyCode::Char('k') | KeyCode::Up => self.swap_up(),
            KeyCode::Char('j') | KeyCode::Down => self.swap_down(),
            _ => {}
        }
    }

    pub fn is_typing(&self) -> bool {
        self.input_mode == InputMode::Typing
    }
}

impl Default for SelectMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyHandler for &mut SelectMenu {
    fn handle_key_code(self, key_code: KeyCode) {
        match self.input_mode {
            InputMode::Navigating => self.handle_nav_key_code(key_code),
            InputMode::Typing => self.handle_type_key_code(key_code),
            InputMode::Grabbing => self.handle_grab_key_code(key_code),
        }
    }
}

impl Widget for &mut SelectMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // TODO: Can probably still use Option here instead
        let sel_ind = if self.input_mode != InputMode::Grabbing {
            self.items.len()
        } else if let Some(i) = self.state.selected() {
            i
        } else {
            self.items.len()
        };

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(ind, path_var)| {
                let mut list_item: ListItem = path_var.into();

                if ind == sel_ind {
                    list_item = list_item.bg(Color::Blue).fg(Color::Gray)
                }

                list_item
            })
            .collect();

        let list = List::new(items).highlight_symbol(">");

        // Confirmation pop-up
        // (confirmation code goes here...)

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&PathVar> for ListItem<'_> {
    fn from(value: &PathVar) -> Self {
        let line = match value.active {
            true => Line::from(format!("   {}", value.path.display())),
            false => Line::styled(format!("   {}", value.path.display()), Color::DarkGray),
        };

        ListItem::new(line)
    }
}
