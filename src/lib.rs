use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    prelude::*,
    symbols::border,
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{env, io};

// Perhaps rename to SelectorWindow?
/// TODO: Doc comment
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    counter: u16,
    sel_menu: SelectMenu,
}

/// TODO: Doc comment
impl App {
    /// TODO: Doc comment
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// TODO: Doc comment
    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// TODO: Doc comment
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_event(event);
            }
            _ => {}
        };
        Ok(())
    }

    /// TODO: Doc comment
    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .sel_menu
            .items
            .iter()
            .map(|it| ListItem::from(it.clone()))
            .collect();

        let list = List::new(items).highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.sel_menu.state);
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

/// TODO: Doc comment
impl Widget for &mut App {
    /// TODO: Doc comment
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Counter App Tutorial".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        // Paragraph::new(counter_text)
        // .centered()
        // .block(block)
        // .render(area, buf);

        self.render_list(area, buf);
    }
}

/// TODO: Doc comment
#[derive(Debug)]
struct SelectMenu {
    items: Vec<String>,
    state: ListState,
}

/// TODO: Doc comment
impl SelectMenu {
    /// TODO: Doc comment
    pub fn new() -> SelectMenu {
        SelectMenu {
            items: read_path().unwrap(),
            state: ListState::default(),
        }
    }
}

/// TODO: Doc comment
// impl StatefulWidget for &SelectMenu {
//     type State = ListState;

//     /// TODO: Doc comment
//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         // TODO: Clone? Really?
//         let list = List::new(self.items.clone());

//         StatefulWidget::render(list, area, buf, state);
//     }
// }

impl Default for SelectMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn read_path() -> io::Result<Vec<String>> {
    Ok(env::var("PATH")
        .expect("PATH variable not present! Shutting down...")
        .split(':')
        .map(|str| String::from(str))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
