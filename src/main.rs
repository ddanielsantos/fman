mod debug;
mod fs;
mod ui;

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
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::DirEntry;
use std::path::PathBuf;
use style::palette::tailwind::SLATE;

#[derive(Parser, Debug, Default)]
#[command(version, long_about = None)]
struct Args {}

#[derive(Debug, Default)]
struct App {
    show_hidden: bool,
    should_quit: bool,
    mode: Mode,
    input: Input,
    left_rect_list: EntriesList,
    queued_items: HashSet<PathBuf>,
}

#[derive(Debug, PartialEq, Default)]
enum Mode {
    #[default]
    Normal,
    Creating,
}

#[derive(Debug, Default)]
struct Input {
    char_index: usize,
    text: String,
}

impl Input {
    fn clear(&mut self) {
        self.text.clear();
        self.char_index = 0;
    }

    fn insert(&mut self, idx: usize, c: char) {
        self.text.insert(idx, c);
    }

    fn move_to_right(&mut self) {
        let new_index = self.char_index.saturating_add(1);
        self.char_index = self.clamp_index(new_index);
    }

    fn clamp_index(&self, new_index: usize) -> usize {
        new_index.clamp(0, self.text.chars().count())
    }

    fn move_to_left(&mut self) {
        let new_index = self.char_index.saturating_sub(1);
        self.char_index = self.clamp_index(new_index);
    }

    fn add_char(&mut self, c: char) {
        let idx = byte_index(&self);
        self.insert(idx, c);
        self.move_to_right();
    }

    fn delete_char(&mut self) {
        let idx = byte_index(&self).saturating_sub(1);

        if self.char_index == 0 {
            return;
        }

        self.text.remove(idx);
        self.move_to_left();
    }
}

#[derive(Debug, Default)]
struct EntriesList {
    items: Vec<DirEntry>,
    state: ListState,
}

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl App {
    pub fn with_args() -> Self {
        Self {
            should_quit: false,
            show_hidden: false,
            mode: Mode::default(),
            input: Input::default(),
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
        let current_path = fs::current_dir().unwrap();

        let [left_rect, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());

        let current_path_content: Vec<String> = self
            .update_content(fs::get_content(&current_path, self.show_hidden))
            .iter()
            .map(fs::dir_entry_to_string)
            .collect();

        let left_block = Block::bordered()
            .title(current_path.display().to_string())
            .border_type(ratatui::widgets::BorderType::Rounded);
        let list = List::new(current_path_content)
            .block(left_block)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(list, left_rect, &mut self.left_rect_list.state);
        frame.render_widget(
            Block::bordered()
                .title("content")
                .border_type(ratatui::widgets::BorderType::Rounded),
            right,
        );

        if self.mode == Mode::Creating {
            let input_popup = ui::Input::new(&self.input).as_popup();
            let cursor_position = input_popup.get_cursor_position(frame.area());
            frame.render_widget(input_popup, frame.area());
            frame.set_cursor_position(cursor_position)
        }
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.mode {
            Mode::Creating => match key.code {
                KeyCode::Enter => self.create_items(),
                KeyCode::Char(c) => self.input.add_char(c),
                KeyCode::Backspace => self.input.delete_char(),
                KeyCode::Left => self.input.move_to_left(),
                KeyCode::Right => self.input.move_to_right(),
                _ => (),
            },
            Mode::Normal => match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Up | KeyCode::Char('j') => self.move_up(),
                KeyCode::Down | KeyCode::Char('k') => self.move_down(),
                KeyCode::Left | KeyCode::Char('h') => self.move_to_parent(),
                KeyCode::Right | KeyCode::Char('l') => self.move_to_child(),
                KeyCode::Char('.') => self.toggle_show_hidden(),
                KeyCode::Char(' ') => self.toggle_presence_on_queue(),
                KeyCode::Char('d') => self.delete_queued_items(),
                KeyCode::Char('n') => self.change_to_creating_mode(),
                _ => (),
            },
        }
    }

    fn move_up(&mut self) {
        self.left_rect_list.state.select_previous()
    }

    fn move_down(&mut self) {
        self.left_rect_list.state.select_next()
    }

    fn select_first(&mut self) {
        self.left_rect_list.state.select_first();
    }

    fn move_to_child(&mut self) {
        if let Some(index) = self.left_rect_list.state.selected() {
            let new_path = &self.left_rect_list.items[index].path();
            if !new_path.is_dir() {
                return;
            }

            if let Err(_r) = fs::change_dir(new_path, || self.select_first()) {
                tracing::error!("Could not move to child dir {:?}", new_path);
            }
        }
    }

    fn move_to_parent(&mut self) {
        let parent = fs::current_dir().unwrap().parent().map(|p| p.to_path_buf());

        if parent.is_none() {
            return;
        }

        let parent = &parent.unwrap();
        if let Err(_r) = fs::change_dir(parent, || self.select_first()) {
            tracing::error!("Could not move to {:?}: {}", parent, _r);
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
        let mut items_to_delete: Vec<PathBuf> = self.queued_items.iter().cloned().collect();

        items_to_delete.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        let current_dir = &fs::current_dir().unwrap();
        if items_to_delete.contains(current_dir) {
            if let Some(parent) = current_dir.parent() {
                if let Err(e) = fs::change_dir(parent, || self.select_first()) {
                    tracing::error!("Error while moving to parent of {:?}: {}", current_dir, e);
                    return;
                }
            }
        }

        fs::delete_all(items_to_delete);
    }

    fn change_to_creating_mode(&mut self) {
        self.mode = Mode::Creating;
    }

    fn create_items(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
    }
}

fn byte_index(input: &Input) -> usize {
    input
        .text
        .char_indices()
        .map(|(i, _)| i)
        .nth(input.char_index)
        .unwrap_or(input.text.len())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let _guard = initialize_logging()?;

    let terminal = ratatui::init();
    let app_result = App::with_args().run(terminal).context("app loop failed");

    ratatui::restore();

    app_result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn idk() {
        let input = Input {
            char_index: 6,
            text: "とラストガエル".to_string(),
        };

        let actual = byte_index(&input);
        assert_eq!(actual, 18);
    }
}
