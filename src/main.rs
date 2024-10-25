use std::env::current_dir;
use std::fs::{self, DirEntry};
use std::path::Path;

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use ratatui::prelude::*;
use ratatui::widgets::{List, ListState};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::Block,
    DefaultTerminal, Frame,
};
use style::palette::tailwind::SLATE;

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
    left_rect_list: EntriesList,
}

#[derive(Debug, Default)]
struct EntriesList {
    items: Vec<DirEntry>,
    state: ListState,
}

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl App {
    pub fn with_args(args: Args) -> Self {
        Self {
            args,
            should_quit: false,
            left_rect_list: EntriesList::default(),
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

    fn draw(&mut self, frame: &mut Frame) {
        let current_dir = current_dir().unwrap().display().to_string();
        let current_path = self.args.path.as_deref().unwrap_or(&current_dir);

        let [left_rect, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

        let current_path_content: Vec<String> = self
            .update_content(get_content(current_path))
            .into_iter()
            .map(|de| de.path().display().to_string())
            .collect();

        let left_block = Block::bordered().title(current_dir);
        let list = List::new(current_path_content)
            .block(left_block)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        frame.render_stateful_widget(list, left_rect, &mut self.left_rect_list.state);
        frame.render_widget(Block::bordered().title("content"), right);
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('j') => self.move_up(),
            KeyCode::Down | KeyCode::Char('k') => self.move_down(),
            KeyCode::Left | KeyCode::Char('h') => self.move_to_parent(),
            KeyCode::Right | KeyCode::Char('l') => self.move_to_child(),
            _ => (),
        }
    }

    fn move_up(&mut self) {
        self.left_rect_list.state.select_previous()
    }

    fn move_down(&mut self) {
        self.left_rect_list.state.select_next()
    }

    fn move_to_child(&mut self) {
        if let Some(index) = self.left_rect_list.state.selected() {
            self.args.path = Some(
                self.left_rect_list.items[index]
                    .path()
                    .display()
                    .to_string(),
            );
        }
    }

    fn move_to_parent(&mut self) {
        if let Some(path) = &self.args.path {
            let path = Path::new(&path);
            let parent = path.parent();
            self.args.path = Some(parent.unwrap().display().to_string());
        }
    }

    fn update_content(&mut self, content: Vec<DirEntry>) -> &Vec<DirEntry> {
        self.left_rect_list.items = content;
        &self.left_rect_list.items
    }
}

fn get_content(path: &str) -> Vec<DirEntry> {
    fs::read_dir(path)
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_else(|_| Vec::new())
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
