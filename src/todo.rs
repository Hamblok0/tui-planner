use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use crate::db::DB;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToDoItem {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub complete: bool,
}

pub fn activate(textarea: &mut TextArea, which: &usize) {
    if let Some(block) = textarea.block() {
        let title = get_title(*which).unwrap();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(title),
        );
    }
}

pub fn deactivate(textarea: &mut TextArea, which: &usize) {
    if let Some(block) = textarea.block() {
        let title = get_title(*which).unwrap();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(Style::default());
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray))
                .title(title),
        );
    }
}

fn get_title(which: usize) -> Option<String> {
    match which {
        0 => Some("Title".to_string()),
        1 => Some("Description".to_string()),
        _ => None,
    }
}

#[derive(Debug)]
pub struct ToDoState {
    pub items: Vec<ToDoItem>,
    pub state: ListState,
}

impl ToDoState {
    pub fn new(items: Vec<ToDoItem>) -> Self {
        Self {
            items,
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        if self.items.len() == 0 {
            return
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.len() == 0 {
            return
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn toggle_complete(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.items[i].complete = !self.items[i].complete;
                // save_session(&self.items);
            }
            None => {}
        }
    }

    pub fn create_task(&mut self, db: &DB, title: String, description: String) {    
        let id = db.create_todo(&title, &description).unwrap();
        
        self.items.push(ToDoItem {
            id,
            title,
            description,
            complete: false,
        });
    }

    pub fn delete_task(&mut self) {
        match self.state.selected() {
            Some(i) => {
                self.items.remove(i);
                self.unselect();
                save_session(&self.items);
            }
            None => {}
        }
    }
    pub fn overwrite_task(&mut self, title: String, description: String) {
        match self.state.selected() {
            Some(i) => {
                let todo = &mut self.items[i];

                if title != todo.title {
                    todo.title = title;
                }
                if description != todo.description {
                    todo.description = description;
                }
                save_session(&self.items);
            }
            None => {}
        }
    }
    pub fn get_selected_todo(&mut self) -> Option<&ToDoItem> {
        match self.state.selected() {
            Some(i) => {
                return Some(&self.items[i]);
            }
            None => None,
        }
    }
}

pub struct ToDoModal<'a> {
    pub textareas: [TextArea<'a>; 2],
    pub which: usize,
    pub mode: ModalMode,
}

#[derive(PartialEq)]
pub enum ModalMode {
    Inactive,
    New,
    View,
    Edit,
}

impl<'a> ToDoModal<'a> {
    pub fn change_focus(&mut self) {
        deactivate(&mut self.textareas[self.which], &mut self.which);
        self.which = (self.which + 1) % 2;
        activate(&mut self.textareas[self.which], &mut self.which);
    }

    pub fn get_center(&self, r: Rect) -> Rect {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ],
        )
        .split(r);

        Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ],
        )
        .split(layout[1])[1]
    }
}
