use ratatui::{
    prelude::Style,
    widgets::*,
};
use tui_textarea::TextArea;

pub struct ToDoItem (pub String, pub bool);

pub struct ToDoState {
    pub items: Vec<ToDoItem>,
    pub state: ListState,
}

impl ToDoState {
    fn new(items: Vec<ToDoItem>) -> Self {
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
                self.items[i].1 = !self.items[i].1;
            }
            None => {}
        }
    }

    pub fn create_task(&mut self, text: String) {
        self.items.push(ToDoItem(text, false));
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
}

pub enum Modal<'a> {
    Inactive,
    Active(TextArea<'a>),
}

impl<'a> Modal<'a> {
    pub fn toggle(&mut self) {
        *self = match self {
            Modal::Inactive => {
                let mut textarea = TextArea::default();

                textarea.set_cursor_line_style(Style::default());
                textarea.set_placeholder_text("Short To-Do Description...");
                Modal::Active(textarea)
            }
            Modal::Active(_) => Modal::Inactive 
        }
    }
}
pub struct App<'a> {
    pub todo: ToDoState,
    pub modal: Modal<'a>, 
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            todo: ToDoState::new(vec![
                ToDoItem("Item 1".to_string(), false),
                ToDoItem("Item 2".to_string(), false),
                ToDoItem("Item 3".to_string(), false),
            ]),
            modal: Modal::Inactive,
        }
    }
}
