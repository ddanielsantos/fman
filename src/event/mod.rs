use ratatui::crossterm::event::KeyCode;

use crate::Mode;
pub use handler::handle_event;
use Event::*;

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
    CancelCreation,
}

pub fn get_event<'a>(mode: &'a Mode, code: &'a KeyCode) -> Event {
    match mode {
        Mode::Normal => match code {
            KeyCode::Char('q') => Quit,
            KeyCode::Up | KeyCode::Char('j') => MoveUp,
            KeyCode::Down | KeyCode::Char('k') => MoveDown,
            KeyCode::Left | KeyCode::Char('h') => MoveToParent,
            KeyCode::Right | KeyCode::Char('l') => MoveToChild,
            KeyCode::Char('.') => ToggleHidden,
            KeyCode::Char(' ') => ToggleQueue,
            KeyCode::Char('d') => DeleteQueue,
            KeyCode::Char('n') => ChangeToCreating,
            KeyCode::Char('?') => ToggleCommands,
            _ => Noop,
        },
        Mode::ShowingCommands => match code {
            KeyCode::Esc | KeyCode::Char('q') => ToggleCommands,
            KeyCode::Up | KeyCode::Char('j') => MoveUp,
            KeyCode::Down | KeyCode::Char('k') => MoveDown,
            KeyCode::Enter => ExecuteCommand,
            _ => Noop,
        },
        Mode::Creating => match code {
            KeyCode::Esc => CancelCreation,
            KeyCode::Enter => ConfirmCreation,
            KeyCode::Char(c) => AddChar(c.to_string()),
            KeyCode::Backspace => DeleteChar,
            KeyCode::Left => MoveLeft,
            KeyCode::Right => MoveRight,
            _ => Noop,
        },
    }
}

fn get_events() -> [Event; 17] {
    [
        Noop,
        DeleteChar,
        MoveLeft,
        MoveRight,
        AddChar("".to_string()),
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
    ]
}

pub fn get_event_name(event: &Event) -> String {
    match event {
        DeleteChar => "delete char",
        MoveLeft => "move left",
        MoveRight => "move right",
        AddChar(_) => "add char",
        Quit => "(q) quit",
        MoveUp => "(↑ | k) move up",
        MoveDown => "(↓ | j) move down",
        MoveToParent => "(← | h) move to parent",
        MoveToChild => "(→ | l) move to child",
        ToggleHidden => "(.) toggle hidden",
        ToggleQueue => "(<Space>) toggle queue",
        DeleteQueue => "(d) delete queue",
        ToggleCommands => "(?) toggle commands",
        ChangeToCreating => "(n) create folder/file",
        ConfirmCreation => "(<Enter>) confirm creation",
        ExecuteCommand => "(<Enter>) execute command",
        CancelCreation => "(<Esc>) cancel creation",
        Noop => "noop",
    }
    .to_string()
}

pub fn in_reexecution_allow_list(event: &Event) -> bool {
    !matches!(
        event,
        Noop | ExecuteCommand
            | AddChar(_)
            | DeleteChar
            | ConfirmCreation
            | ToggleCommands
            | CancelCreation
            | MoveLeft
            | MoveRight
    )
}

pub fn get_command_picker_events() -> Vec<Event> {
    get_events()
        .into_iter()
        .filter(in_reexecution_allow_list)
        .collect()
}
