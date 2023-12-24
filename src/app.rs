use ratatui::widgets::*;

struct ToDoState<T> {
    items: Vec<T>,
    state: ListState,
}

impl<T> ToDoState<T> {
    fn new(items: Vec<T>) -> Self {
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
}

pub struct App<'a> {
    todo: ToDoState<(&'a str, bool)>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            todo: ToDoState::new(vec![
                ("Item 1", false),
                ("Item 2", false),
                ("Item 3", false),
            ]),
        }
    }
}
