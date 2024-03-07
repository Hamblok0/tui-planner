use ratatui::{prelude::*, widgets::*};
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct ToDoItem {
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
            }
            None => {}
        }
    }

    pub fn create_task(&mut self, title: String, description: String) {
        self.items.push(ToDoItem {
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

pub enum Modal<'a> {
    Inactive,
    Active([TextArea<'a>; 2], usize, ModalType),
}

#[derive(PartialEq)]
pub enum ModalType {
    Inactive,
    New,
    View,
    Edit,
}

impl<'a> Modal<'a> {
    pub fn change_focus(&mut self) {
        if let Modal::Active(ref mut textareas, ref mut which, _) = self {
            deactivate(&mut textareas[*which], which);
            *which = (*which + 1) % 2;
            activate(&mut textareas[*which], which);
        }
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