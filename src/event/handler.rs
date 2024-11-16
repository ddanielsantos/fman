use std::path::PathBuf;

use crate::event::Event;
use crate::fs::*;
use crate::App;
use crate::Input;
use crate::Mode;

pub fn handle_event(event: &Event, app: &mut App) {
    match event {
        Event::DeleteChar => delete_char(&mut app.input),
        Event::MoveLeft => move_to_left(&mut app.input),
        Event::MoveRight => move_to_right(&mut app.input),
        Event::AddChar(c) => add_char(&mut app.input, c),
        Event::Quit => quit(app),
        Event::MoveUp => move_up(app),
        Event::MoveDown => move_down(app),
        Event::MoveToParent => move_to_parent(app),
        Event::MoveToChild => move_to_child(app),
        Event::ToggleHidden => toggle_show_hidden(app),
        Event::ToggleQueue => toggle_presence_on_queue(app),
        Event::DeleteQueue => delete_queued_items(app),
        Event::ToggleCommands => toggle_show_commands(app),
        Event::ExecuteCommand => execute_command(app),
        Event::ChangeToCreating => change_to_creating_mode(app),
        Event::ConfirmCreation => create_items(app),
        Event::Noop => {}
    }
}

fn execute_command(app: &mut App) {
    if let Some(index) = app.command_list.state.selected() {
        let second_hand_event = app.command_list.items[index].clone();

        match second_hand_event {
            Event::Noop | Event::ExecuteCommand | Event::AddChar(_) | Event::ToggleCommands => {}
            _ => {
                handle_event(&second_hand_event, app);
            }
        }

        toggle_show_commands(app);
    }
}

fn create_items(app: &mut App) {
    app.mode = Mode::Normal;

    create_path(&app.input.text);

    clear(&mut app.input);
}

fn move_to_child(app: &mut App) {
    if let Some(index) = app.left_rect_list.state.selected() {
        let new_path = &app.left_rect_list.items[index].path();
        if !new_path.is_dir() {
            return;
        }

        if let Err(_r) = change_dir(new_path, || app.left_rect_list.state.select_first()) {
            tracing::error!("Could not move to child dir {:?}", new_path);
        }
    }
}

fn move_to_parent(app: &mut App) {
    let parent = current_dir().unwrap().parent().map(|p| p.to_path_buf());

    if parent.is_none() {
        return;
    }

    let parent = &parent.unwrap();
    if let Err(_r) = change_dir(parent, || app.left_rect_list.state.select_first()) {
        tracing::error!("Could not move to {:?}: {}", parent, _r);
    }
}

fn toggle_show_hidden(app: &mut App) {
    app.show_hidden = !app.show_hidden;
}

fn toggle_presence_on_queue(app: &mut App) {
    if let Some(index) = app.left_rect_list.state.selected() {
        let item = app.left_rect_list.items[index].path();
        if app.queued_items.contains(&item) {
            app.queued_items.remove(&item);
        } else {
            app.queued_items.insert(item);
        }
    }
}

fn delete_queued_items(app: &mut App) {
    let mut items_to_delete: Vec<PathBuf> = app.queued_items.iter().cloned().collect();

    items_to_delete.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    let current_dir = &current_dir().unwrap();
    if items_to_delete.contains(current_dir) {
        if let Some(parent) = current_dir.parent() {
            if let Err(e) = change_dir(parent, || app.left_rect_list.state.select_first()) {
                tracing::error!("Error while moving to parent of {:?}: {}", current_dir, e);
                return;
            }
        }
    }

    delete_all(items_to_delete);
}

fn change_to_creating_mode(app: &mut App) {
    app.mode = Mode::Creating;
    app.input = Input::new(current_dir().unwrap().display().to_string())
}

fn toggle_show_commands(app: &mut App) {
    if app.mode != Mode::ShowingCommands {
        app.mode = Mode::ShowingCommands;
    } else {
        app.mode = Mode::Normal;
    }
    tracing::debug!("{:?}", app.mode);
}

fn move_down(app: &mut App) {
    if app.mode == Mode::Normal {
        app.left_rect_list.state.select_next()
    } else if app.mode == Mode::ShowingCommands {
        app.command_list.state.select_next()
    }
}

fn move_up(app: &mut App) {
    if app.mode == Mode::Normal {
        app.left_rect_list.state.select_previous()
    } else if app.mode == Mode::ShowingCommands {
        app.command_list.state.select_previous()
    }
}

fn quit(app: &mut App) {
    app.should_quit = true;
}

fn delete_char(input: &mut Input) {
    let idx = byte_index(input).saturating_sub(1);

    if input.char_index == 0 {
        return;
    }

    input.text.remove(idx);
    move_to_left(input);
}

fn clear(input: &mut Input) {
    input.text.clear();
    input.char_index = 0;
}

fn insert(input: &mut Input, idx: usize, c: &str) {
    let c = c.chars().next().unwrap();

    input.text.insert(idx, c);
}

fn move_to_right(input: &mut Input) {
    let new_index = input.char_index.saturating_add(1);
    input.char_index = clamp_index(input, new_index);
}

fn clamp_index(input: &Input, new_index: usize) -> usize {
    new_index.clamp(0, input.text.chars().count())
}

fn move_to_left(input: &mut Input) {
    let new_index = input.char_index.saturating_sub(1);
    input.char_index = clamp_index(input, new_index);
}

fn add_char(input: &mut Input, c: &str) {
    let idx = byte_index(input);
    insert(input, idx, c);
    move_to_right(input);
}

fn byte_index(input: &Input) -> usize {
    input
        .text
        .char_indices()
        .map(|(i, _)| i)
        .nth(input.char_index)
        .unwrap_or(input.text.len())
}
