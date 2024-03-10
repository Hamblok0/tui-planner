use tui_textarea::{Input, Key};

use crate::app::*;
use crate::todo::*;
use crate::local_data::save_session;

pub fn key_events(app: &mut App) -> Option<usize> {
    if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
        if key.kind == crossterm::event::KeyEventKind::Press {
            match app.modal {
                Modal::Active(ref mut textareas, which, ref modal_type) => match modal_type {
                    ModalType::New => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalType::Inactive);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.todo.create_task(
                                textareas[0].lines().join(""),
                                textareas[1].lines().join(""),
                            );
                            app.toggle_modal(ModalType::Inactive);
                        }
                        Input { key: Key::Tab, .. } => {
                            app.modal.change_focus();
                        }
                        input => {
                            textareas[which].input(input);
                        }
                    },
                    ModalType::View => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalType::Inactive);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.toggle_modal(ModalType::Inactive);
                        }
                        Input {
                            key: Key::Char('e'),
                            ..
                        } => {
                            app.toggle_modal(ModalType::Edit);
                        }
                        input => {}
                    },
                    ModalType::Edit => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.toggle_modal(ModalType::View);
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.todo.overwrite_task(
                                textareas[0].lines().join(""),
                                textareas[1].lines().join(""),
                            );
                            app.toggle_modal(ModalType::View);
                        }
                        Input { key: Key::Tab, .. } => {
                            app.modal.change_focus();
                        }
                        input => {
                            textareas[which].input(input);
                        }
                    },
                    _ => {}
                },
                Modal::Inactive => match key.code {
                    crossterm::event::KeyCode::Char('q') => return Some(0),
                    crossterm::event::KeyCode::Char('c') => app.todo.toggle_complete(),
                    crossterm::event::KeyCode::Char('j') => app.todo.next(),
                    crossterm::event::KeyCode::Char('k') => app.todo.previous(),
                    crossterm::event::KeyCode::Char('d') => app.todo.delete_task(),
                    crossterm::event::KeyCode::Char('n') => app.toggle_modal(ModalType::New),
                    crossterm::event::KeyCode::Char('v') => app.toggle_modal(ModalType::View),
                    crossterm::event::KeyCode::Char('e') => match app.todo.state.selected() {
                        Some(_) => app.toggle_modal(ModalType::Edit),
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
