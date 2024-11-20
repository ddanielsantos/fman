use ratatui::crossterm::event::KeyCode;

use crate::Mode;
pub use handler::handle_event;

mod handler;

#[derive(Debug, Clone)]
pub enum Event {
    Noop,
    DeleteChar,
    MoveLeft,
    MoveRight,
    AddChar(String),
    Quit,
    MoveUp,
    MoveDown,
    MoveToParent,
    MoveToChild,
    ToggleHidden,
    ToggleQueue,
    DeleteQueue,
    ToggleCommands,
    ChangeToCreating,
    ConfirmCreation,
    ExecuteCommand,
}

pub fn get_event<'a>(mode: &'a Mode, code: &'a KeyCode) -> Event {
    match mode {
        Mode::Normal => match code {
            KeyCode::Char('q') => Event::Quit,
            KeyCode::Up | KeyCode::Char('j') => Event::MoveUp,
            KeyCode::Down | KeyCode::Char('k') => Event::MoveDown,
            KeyCode::Left | KeyCode::Char('h') => Event::MoveToParent,
            KeyCode::Right | KeyCode::Char('l') => Event::MoveToChild,
            KeyCode::Char('.') => Event::ToggleHidden,
            KeyCode::Char(' ') => Event::ToggleQueue,
            KeyCode::Char('d') => Event::DeleteQueue,
            KeyCode::Char('n') => Event::ChangeToCreating,
            KeyCode::Char('?') => Event::ToggleCommands,
            _ => Event::Noop,
        },
        Mode::ShowingCommands => match code {
            KeyCode::Esc | KeyCode::Char('q') => Event::ToggleCommands,
            KeyCode::Up | KeyCode::Char('j') => Event::MoveUp,
            KeyCode::Down | KeyCode::Char('k') => Event::MoveDown,
            KeyCode::Enter => Event::ExecuteCommand,
            _ => Event::Noop,
        },
        Mode::Creating => match code {
            KeyCode::Enter => Event::ConfirmCreation,
            KeyCode::Char(c) => Event::AddChar(c.to_string()),
            KeyCode::Backspace => Event::DeleteChar,
            KeyCode::Left => Event::MoveLeft,
            KeyCode::Right => Event::MoveRight,
            _ => Event::Noop,
        },
    }
}

fn get_events() -> [Event; 15] {
    [
        Event::Noop,
        Event::DeleteChar,
        Event::MoveLeft,
        Event::MoveRight,
        Event::AddChar("".to_string()),
        Event::Quit,
        Event::MoveUp,
        Event::MoveDown,
        Event::MoveToParent,
        Event::MoveToChild,
        Event::ToggleHidden,
        Event::ToggleQueue,
        Event::DeleteQueue,
        Event::ToggleCommands,
        Event::ExecuteCommand,
    ]
}

pub fn get_event_name(event: &Event) -> String {
    match event {
        Event::DeleteChar => "delete_char (d)",
        Event::MoveLeft => "move_left (<-, h)",
        Event::MoveRight => "move_right",
        Event::AddChar(_) => "add_char",
        Event::Quit => "quit",
        Event::MoveUp => "move_up",
        Event::MoveDown => "move_down",
        Event::MoveToParent => "move_to_parent",
        Event::MoveToChild => "move_to_child",
        Event::ToggleHidden => "toggle_hidden",
        Event::ToggleQueue => "toggle_queue",
        Event::DeleteQueue => "delete_queue",
        Event::ToggleCommands => "toggle_commands",
        Event::ChangeToCreating => "change_to_creating",
        Event::ConfirmCreation => "confirm_creation",
        Event::ExecuteCommand => "execute_command",
        Event::Noop => "noop",
    }
    .to_string()
}

pub fn in_reexecution_allow_list(event: &Event) -> bool {
    match event {
        Event::Noop
        | Event::ExecuteCommand
        | Event::AddChar(_)
        | Event::ToggleCommands
        | Event::MoveLeft
        | Event::MoveRight => false,
        _ => true,
    }
}

pub fn get_command_picker_events() -> Vec<Event> {
    get_events()
        .into_iter()
        .filter(in_reexecution_allow_list)
        .collect()
}
