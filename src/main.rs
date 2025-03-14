use std::env;
use std::io::Result;

use pathui;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use clap::Parser;

/// Program to manage user PATH
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// .path file to load
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    let _args = Args::parse();

    let mut app = pathui::App::default();

    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();

    result
}
