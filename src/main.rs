mod debug;

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::{self, DirEntry};
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use debug::initialize_logging;
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
struct Args {}

#[derive(Debug, Default)]
struct App {
    show_hidden: bool,
    should_quit: bool,
    left_rect_list: EntriesList,
    queued_items: HashSet<PathBuf>,
}

#[derive(Debug, Default)]
struct EntriesList {
    items: Vec<DirEntry>,
    state: ListState,
}

const WINDOWS_FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl App {
    pub fn with_args() -> Self {
        Self {
            should_quit: false,
            show_hidden: false,
            left_rect_list: EntriesList::default(),
            queued_items: HashSet::new(),
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
        let current_path = std::env::current_dir().unwrap().display().to_string();

        let [left_rect, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

        let current_path_content: Vec<String> = self
            .update_content(get_content(&current_path, self.show_hidden))
            .iter()
            .map(dir_entry_to_string)
            .collect();

        let left_block = Block::bordered().title(current_path);
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
            KeyCode::Char('.') => self.toggle_show_hidden(),
            KeyCode::Char(' ') => self.toggle_presence_on_queue(),
            KeyCode::Char('d') => self.delete_queued_items(),
            _ => (),
        }
    }

    fn move_up(&mut self) {
        self.left_rect_list.state.select_previous()
    }

    fn move_down(&mut self) {
        self.left_rect_list.state.select_next()
    }

    fn change_dir(&mut self, new_path: PathBuf) -> Result<()> {
        let res = std::env::set_current_dir(new_path).wrap_err("fuck");
        self.left_rect_list.state.select_first();

        res
    }

    fn move_to_child(&mut self) {
        if let Some(index) = self.left_rect_list.state.selected() {
            let new_path = self.left_rect_list.items[index].path();
            let cloned = new_path.clone();

            if !new_path.is_dir() {
                return;
            }

            if let Err(_r) = self.change_dir(new_path) {
                tracing::error!("Could not move to child dir {:?}", cloned);
            }
        }
    }

    fn move_to_parent(&mut self) {
        let parent = std::env::current_dir()
            .unwrap()
            .parent()
            .map(|p| p.to_path_buf());

        if parent.is_none() {
            return;
        }

        let parent = parent.unwrap();

        let cloned = parent.clone();

        if let Err(_r) = self.change_dir(parent) {
            tracing::error!("Could not move to {:?}: {}", cloned, _r);
        }
    }

    fn update_content(&mut self, content: Vec<DirEntry>) -> &Vec<DirEntry> {
        self.left_rect_list.items = content;
        &self.left_rect_list.items
    }

    fn toggle_show_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
    }

    fn toggle_presence_on_queue(&mut self) {
        if let Some(index) = self.left_rect_list.state.selected() {
            let item = self.left_rect_list.items[index].path();
            if self.queued_items.contains(&item) {
                self.queued_items.remove(&item);
            } else {
                self.queued_items.insert(item);
            }
        }
    }

    fn delete_queued_items(&mut self) {
        self.queued_items.iter().for_each(|qi| {
            if qi.is_file() {
                let _ = std::fs::remove_file(qi);
            }

            if qi.is_dir() {
                let _ = std::fs::remove_dir_all(qi);
            }
        })
    }
}

fn dir_entry_to_string(de: &DirEntry) -> String {
    de.path().display().to_string()
}

fn get_content(path: &str, show_hidden: bool) -> Vec<DirEntry> {
    fs::read_dir(path)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|de| {
                    if show_hidden {
                        true
                    } else {
                        is_not_hidden(&de.path())
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}

fn is_not_hidden(path: &PathBuf) -> bool {
    !is_hidden(path)
}

fn is_hidden(path: &PathBuf) -> bool {
    let md = std::fs::metadata(path);

    match md {
        Ok(md) => {
            if cfg!(target_os = "windows") {
                md.file_attributes() & WINDOWS_FILE_ATTRIBUTE_HIDDEN != 0
            } else {
                path.to_string_lossy().starts_with(".")
            }
        }
        Err(md_error) => {
            tracing::warn!("{}", md_error);
            false
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let _guard = initialize_logging()?;

    let terminal = ratatui::init();
    let app_result = App::with_args().run(terminal).context("app loop failed");

    ratatui::restore();

    app_result
}
