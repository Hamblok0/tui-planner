use ratatui::{prelude::*, widgets::*};
use tui_textarea::TextArea;

use crate::todo::*;
use crate::local_data::load_session;

#[derive(PartialEq)]
pub enum View {
    Main,
    Modal
}

pub struct App<'a> {
    pub todo: ToDoState,
    pub modal: Modal<'a>,
    pub view: View
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let items = match load_session() {
            Some(data) => data,
            None => vec![]
        };
        App {
            todo: ToDoState::new(items),
            modal: Modal::Inactive,
            view: View::Main
        }
    }

    pub fn toggle_modal(&mut self, modal_type: ModalType) {
        let mut textarea = [TextArea::default(), TextArea::default()];
        let which: usize = 0;
        self.modal = match modal_type {
            ModalType::New => {
                textarea[0].set_cursor_line_style(Style::default());
                textarea[0].set_placeholder_text("Short To-Do Description...");
                textarea[0].set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Title")
                        .style(Style::default()),
                );
                textarea[1].set_placeholder_text("Details (Optional)...");
                textarea[1].set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Description")
                        .style(Style::default().fg(Color::DarkGray)),
                );

                Modal::Active(textarea, which, ModalType::New)
            }
            ModalType::View => match self.todo.get_selected_todo() {
                Some(todo) => {
                    textarea[0].set_cursor_line_style(Style::default());
                    textarea[0].set_cursor_style(Style::default());
                    textarea[0].set_placeholder_text("Short To-Do Description...");
                    textarea[0].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Title")
                            .style(Style::default()),
                    );
                    textarea[1].set_cursor_line_style(Style::default());
                    textarea[1].set_cursor_style(Style::default());
                    textarea[1].set_placeholder_text("Details (Optional)...");
                    textarea[1].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Description")
                            .style(Style::default()),
                    );
                    textarea[0].insert_str(&todo.title);
                    textarea[1].insert_str(&todo.description);

                    Modal::Active(textarea, which, ModalType::View)
                }
                None => {
                    panic!("Error: ToDoItem did not return a value")
                }
            },
            ModalType::Edit => match self.todo.get_selected_todo() {
                Some(todo) => {
                    textarea[0].set_cursor_line_style(Style::default());
                    textarea[0].set_placeholder_text("Short To-Do Description...");
                    textarea[0].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Title")
                            .style(Style::default()),
                    );
                    textarea[1].set_placeholder_text("Details (Optional)...");
                    textarea[1].set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Description")
                            .style(Style::default().fg(Color::DarkGray)),
                    );
                    textarea[0].insert_str(&todo.title);
                    textarea[1].insert_str(&todo.description);

                    Modal::Active(textarea, which, ModalType::Edit)
                }
                None => {
                    panic!("Error: ToDoItem did not return a value")
                }
            },
            _ => Modal::Inactive,
        }
    }
}
