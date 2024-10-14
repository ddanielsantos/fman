use std::env::current_dir;
use std::fs::{self, DirEntry};

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use ratatui::prelude::*;
use ratatui::widgets::{List, ListItem};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Block,
    DefaultTerminal, Frame,
};

#[derive(Parser, Debug, Default)]
#[command(version, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Debug, Default)]
struct App {
    args: Args,
    should_quit: bool,
}

enum Action {
    Enter,
    Quit,
    SelectUnselect,
    Delete,
    Trash,
    Copy,
    Paste,
    Move,
}

struct MyItem(DirEntry);

impl From<&MyItem> for ListItem<'_> {
    fn from(value: &MyItem) -> Self {
        let a = format!("{:?}", value.0.path());
        ListItem::new(a)
    }
}

impl App {
    pub fn with_args(args: Args) -> Self {
        Self {
            args,
            should_quit: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let c_dirr = current_dir().expect("idk").into_os_string();
        let current_dir = c_dirr.to_str().expect("idk 2");
        let path = self.args.path.as_deref().unwrap_or(current_dir);

        let [left_rect, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

        let path_content: Vec<String> = fs::read_dir(path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter_map(|e| e.path().to_str().map(String::from))
                    .collect()
            })
            .unwrap_or_else(|_| Vec::new());

        let left_block = Block::bordered().title(path);
        let list = List::new(path_content).block(left_block);
        frame.render_widget(list, left_rect);
        frame.render_widget(Block::bordered().title("content"), right);
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if let KeyCode::Char('q') = key.code {
            self.should_quit = true
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let terminal = ratatui::init();
    let app_result = App::with_args(args)
        .run(terminal)
        .context("app loop failed");
    ratatui::restore();
    app_result
}
