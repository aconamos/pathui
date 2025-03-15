use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    style::palette::tailwind::SLATE,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, ffi::OsStr, io, path::PathBuf};

use crate::InputMode;

/// Widget containing a list of selectable items and a corresponding state.
///
/// To achieve stateful behavior, a [`List`] is created each render and rendered statefully using `self.state`.
#[derive(Debug)]
pub struct SelectMenu {
    items: Vec<PathVar>,
    state: ListState,
    pub highlight_mode: InputMode,
}

/// Represents a path, either on or off.
#[derive(Debug)]
struct PathVar {
    path: PathBuf,
    active: bool,
}

impl PathVar {
    /// Constructs a new PathVar with the given path and set to true.
    fn new(path: PathBuf) -> Self {
        PathVar {
            path: path,
            active: true,
        }
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
            highlight_mode: InputMode::Navigating,
        }
    }

    pub fn sel_next(&mut self) {
        self.state.select_next();
    }

    pub fn sel_prev(&mut self) {
        self.state.select_previous();
    }

    pub fn sel_first(&mut self) {
        self.state.select_first();
    }

    pub fn sel_last(&mut self) {
        self.state.select_last();
    }

    /// Toggle the status of the selected list item
    pub fn toggle_status(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items[i].active = !self.items[i].active
        }
    }

    /// Writes the given path contained within `self.items` to the environment.
    pub fn write_path_to_env(&self) -> io::Result<()> {
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

    /// Gets the x-value corresponding the position of the cursor
    pub fn get_cursor_ind(&mut self) -> Option<(usize, usize)> {
        if let Some(i) = self.state.selected() {
            let path_var = &mut self.items[i];

            let os_str = path_var.path.as_mut_os_str().to_owned();

            return Some((os_str.len(), i));
        }

        None
    }

    /// Appends to the selected path.
    pub fn add_char_to_sel_path(&mut self, c: char) {
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
    pub fn del_char_from_sel_path(&mut self) {
        if let Some(i) = self.state.selected() {
            let path_var = &mut self.items[i];

            // Convert PathVar into a string, push to string, put back
            let os_str = path_var.path.to_str().unwrap();
            if os_str.len() == 0 {
                return;
            }
            let (os_str, _) = os_str.split_at(os_str.len() - 1);
            if os_str.len() == 0 {
                path_var.active = false;
            }
            self.items[i].path = os_str.into();
        }
    }

    /// TODO: Doc comment
    pub fn swap_up(&mut self) {
        if let Some(i) = self.state.selected() {
            if i == 0 {
                return;
            }

            self.items.swap(i, i - 1);

            self.sel_prev();
        }
    }

    /// TODO: Doc comment
    pub fn swap_down(&mut self) {
        if let Some(i) = self.state.selected() {
            if i == self.items.len() - 1 {
                return;
            }

            self.items.swap(i, i + 1);

            self.sel_next();
        }
    }
}

impl Default for SelectMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &mut SelectMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut items: Vec<ListItem> = self.items.iter().map(|it| ListItem::from(it)).collect();

        if self.highlight_mode == InputMode::Grabbing {
            // This if let might be technically redundant, because something should always
            // be selected. If something isn't selected, our App should make sure we don't
            // end up running code in SelectMenu (don't let enter Edit/Grab mode).
            if let Some(i) = self.state.selected() {
                // Stupid hack. Why need clone?
                items[i] = items[i].clone().bg(Color::DarkGray)
            }
        }

        let list = List::new(items).highlight_symbol("> ");

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&PathVar> for ListItem<'_> {
    fn from(value: &PathVar) -> Self {
        let line = match value.active {
            true => Line::from(format!(" ✓ {}", value.path.display())),
            false => Line::styled(format!("   {}", value.path.display()), Color::DarkGray),
        };

        ListItem::new(line)
    }
}
