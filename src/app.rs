use ratatui::{prelude::*, widgets::*};
use rusqlite::Connection;
use tui_textarea::TextArea;

use crate::db::DB;
use crate::local_data::load_session;
use crate::todo::*;

pub enum View<'a> {
    Main,
    Modal(ToDoModal<'a>),
}

pub struct App<'a> {
    pub todo: ToDoState,
    pub view: View<'a>,
    pub db: DB,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let db: DB = DB::new();

        let items = match load_session() {
            Some(data) => data,
            None => vec![],
        };
        App {
            todo: ToDoState::new(items),
            view: View::Main,
            db,
        }
    }

    pub fn toggle_modal(&mut self, modal_mode: ModalMode) {
        let mut textareas = [TextArea::default(), TextArea::default()];
        let which: usize = 0;
        self.view = match modal_mode {
            ModalMode::New => {
                textareas[0].set_cursor_line_style(Style::default());
                textareas[0].set_placeholder_text("Short To-Do Description...");
                textareas[0].set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Title")
                        .style(Style::default()),
                );
                textareas[1].set_placeholder_text("Details (Optional)...");
                textareas[1].set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Description")
                        .style(Style::default().fg(Color::DarkGray)),
                );

                View::Modal(ToDoModal {
                    textareas,
                    which,
                    mode: ModalMode::New,
                })
            }
            ModalMode::View => match self.todo.get_selected_todo() {
                Some(todo) => {
                    textareas[0].set_cursor_line_style(Style::default());
                    textareas[0].set_cursor_style(Style::default());
                    textareas[0].set_placeholder_text("Short To-Do Description...");
                    textareas[0].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Title")
                            .style(Style::default()),
                    );
                    textareas[1].set_cursor_line_style(Style::default());
                    textareas[1].set_cursor_style(Style::default());
                    textareas[1].set_placeholder_text("Details (Optional)...");
                    textareas[1].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Description")
                            .style(Style::default()),
                    );
                    textareas[0].insert_str(&todo.title);
                    textareas[1].insert_str(&todo.description);

                    View::Modal(ToDoModal {
                        textareas,
                        which,
                        mode: ModalMode::View,
                    })
                }
                None => {
                    panic!("Error: ToDoItem did not return a value")
                }
            },
            ModalMode::Edit => match self.todo.get_selected_todo() {
                Some(todo) => {
                    textareas[0].set_cursor_line_style(Style::default());
                    textareas[0].set_placeholder_text("Short To-Do Description...");
                    textareas[0].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Title")
                            .style(Style::default()),
                    );
                    textareas[1].set_placeholder_text("Details (Optional)...");
                    textareas[1].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Description")
                            .style(Style::default().fg(Color::DarkGray)),
                    );
                    textareas[0].insert_str(&todo.title);
                    textareas[1].insert_str(&todo.description);

                    View::Modal(ToDoModal {
                        textareas,
                        which,
                        mode: ModalMode::Edit,
                    })
                }
                None => {
                    panic!("Error: ToDoItem did not return a value")
                }
            },
            _ => View::Main,
        }
    }
}
