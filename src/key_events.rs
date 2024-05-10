use crossterm::event::KeyEvent;
use tui_textarea::{Input, Key};

use crate::app::*;
use crate::db::*;
use crate::todo::*;

pub fn key_events(app: &mut App) -> Option<usize> {
    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
        if key.kind == crossterm::event::KeyEventKind::Press {
            match app.view {
                View::Modal(ref mut modal) => match modal.mode {
                    ModalMode::New => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalMode::Inactive);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.todo.create_task(
                                &app.db,
                                modal.textareas[0].lines().join(""),
                                modal.textareas[1].lines().join(""),
                            );
                            app.toggle_modal(ModalMode::Inactive);
                        }
                        Input { key: Key::Tab, .. } => {
                            modal.change_focus();
                        }
                        input => {
                            modal.textareas[modal.which].input(input);
                        }
                    },
                    ModalMode::View => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalMode::Inactive);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.toggle_modal(ModalMode::Inactive);
                        }
                        Input {
                            key: Key::Char('e'),
                            ..
                        } => {
                            app.toggle_modal(ModalMode::Edit);
                        }
                        input => {}
                    },
                    ModalMode::Edit => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalMode::View);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.todo.overwrite_task(
                                modal.textareas[0].lines().join(""),
                                modal.textareas[1].lines().join(""),
                            );
                            app.toggle_modal(ModalMode::View);
                        }
                        Input { key: Key::Tab, .. } => {
                            modal.change_focus();
                        }
                        input => {
                            modal.textareas[modal.which].input(input);
                        }
                    },
                    _ => {}
                },
                View::Main => match key.code {
                    crossterm::event::KeyCode::Char('q') => return Some(0),
                    crossterm::event::KeyCode::Char('c') => app.todo.toggle_complete(),
                    crossterm::event::KeyCode::Char('j') => app.todo.next(),
                    crossterm::event::KeyCode::Char('k') => app.todo.previous(),
                    crossterm::event::KeyCode::Char('d') => app.todo.delete_task(),
                    crossterm::event::KeyCode::Char('n') => app.toggle_modal(ModalMode::New),
                    crossterm::event::KeyCode::Char('v') => app.toggle_modal(ModalMode::View),
                    crossterm::event::KeyCode::Char('e') => match app.todo.state.selected() {
                        Some(_) => app.toggle_modal(ModalMode::Edit),
                        None => {}
                    },
                    crossterm::event::KeyCode::Char('s') => save_session(&app.todo.items),
                    _ => {}
                },
                _ => (),
            }
        }
    }
    None
}
