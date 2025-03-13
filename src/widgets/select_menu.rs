use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    style::palette::tailwind::SLATE,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, ffi::OsStr, io, path::PathBuf};

/// Widget containing a list of selectable items and a corresponding state.
///
/// To achieve stateful behavior, a `List` is created each render and rendered statefully using `self.state`.
#[derive(Debug)]
pub struct SelectMenu {
    items: Vec<PathVar>,
    state: ListState,
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
    /// Creates a new `SelectMenu` with the contents of the environment PATH variable and a default `ListState`.
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
        let items: Vec<ListItem> = self.items.iter().map(|it| ListItem::from(it)).collect();

        let list = List::new(items).highlight_symbol("> ");

        // Do we really need to be using state here?
        // It feels right on paper, but wrong in code.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&PathVar> for ListItem<'_> {
    fn from(value: &PathVar) -> Self {
        let line = match value.active {
            true => Line::from(format!(" ✓ {}", value.path.display())),
            false => Line::styled(format!("   {}", value.path.display()), SLATE.c200),
        };

        ListItem::new(line)
    }
}
