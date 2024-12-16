mod debug;
mod event;
mod fs;
mod ui;

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use debug::initialize_logging;
use event::Event;
use ratatui::crossterm::event::read;
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Clear, ListState, Paragraph};
use ratatui::{crossterm::event::KeyEventKind, widgets::Block, DefaultTerminal, Frame};
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
    items: Vec<Event>,
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
            if let Key(key) = read()? {
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
                let input = tui_input::Input::new(self.input.text.clone());
                let area = frame.area();
                let rect = Rect {
                    x: area.width / 4,
                    y: area.height / 3,
                    width: area.width / 2,
                    height: 3,
                };
                let delimiter = fs::get_delimiter();
                let block = Block::bordered()
                    .title_top(format!("create item or folders ({} ended)", delimiter))
                    .border_type(ratatui::widgets::BorderType::Rounded);
                let scroll_offset = rect.width - 2;
                let scroll = input.visual_scroll(scroll_offset.into()) as u16;
                let p = Paragraph::new(input.value())
                    .block(block)
                    .scroll((0, scroll));

                frame.render_widget(Clear, rect);
                frame.render_widget(p, rect);
            }
            Mode::ShowingCommands => {
                let events = event::get_command_picker_events();
                let command_selector =
                    ui::CommandPicker::new(events.iter().map(event::get_event_name).collect());

                self.command_list.items = events;
                frame.render_stateful_widget(
                    command_selector,
                    frame.area(),
                    &mut self.command_list.state,
                );
            }
            _ => (),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
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
