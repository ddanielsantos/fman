mod debug;
mod event;
mod fs;
mod ui;

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use debug::initialize_logging;
use ratatui::crossterm::event::read;
use ratatui::prelude::*;
use ratatui::widgets::ListState;
use ratatui::{
    crossterm::event::{self as ctevent, Event as CtEvent, KeyEventKind},
    widgets::Block,
    DefaultTerminal, Frame,
};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::DirEntry;
use std::path::PathBuf;

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
    command_list: CommandList,
}

#[derive(Debug, PartialEq, Default)]
enum Mode {
    #[default]
    Normal,
    Creating,
    ShowingCommands,
}

#[derive(Debug, Default)]
struct Input {
    char_index: usize,
    text: String,
}

#[derive(Debug, Default)]
struct CommandList {
    state: ListState,
}

impl Input {
    fn new(text: String) -> Self {
        Self {
            char_index: text.len(),
            text,
        }
    }
}

#[derive(Debug, Default)]
struct EntriesList {
    items: Vec<DirEntry>,
    state: ListState,
}

impl App {
    pub fn with_args() -> Self {
        Self {
            should_quit: false,
            show_hidden: false,
            mode: Mode::default(),
            input: Input::default(),
            left_rect_list: EntriesList::default(),
            queued_items: HashSet::new(),
            command_list: CommandList::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            if let CtEvent::Key(key) = read()? {
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

        let list = ui::MainList::new(current_path.display().to_string(), current_path_content);

        frame.render_stateful_widget(list, left_rect, &mut self.left_rect_list.state);
        frame.render_widget(
            Block::bordered()
                .title("content")
                .border_type(ratatui::widgets::BorderType::Rounded),
            right,
        );

        match self.mode {
            Mode::Creating => {
                let input_popup = ui::Input::new(&self.input).popup();
                let cursor_position = input_popup.get_cursor_position(frame.area());
                frame.render_widget(input_popup, frame.area());
                frame.set_cursor_position(cursor_position)
            }
            Mode::ShowingCommands => {
                let command_selector = ui::CommandPicker::new(event::get_event_names());
                frame.render_stateful_widget(
                    command_selector,
                    frame.area(),
                    &mut self.command_list.state,
                );
            }
            _ => (),
        }
    }

    fn handle_key(&mut self, key: ctevent::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        let event = event::get_event(&self.mode, &key.code);
        event::handle_event(&event, self);
    }

    fn update_content(&mut self, content: Vec<DirEntry>) -> &Vec<DirEntry> {
        self.left_rect_list.items = content;
        &self.left_rect_list.items
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
