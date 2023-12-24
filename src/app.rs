use ratatui::widgets::*;

pub struct ToDoItem<'a>(pub &'a str, pub bool);

pub struct ToDoState<'a> {
    pub items: Vec<ToDoItem<'a>>,
    pub state: ListState,
}

impl<'a> ToDoState<'a>{
    fn new(items: Vec<ToDoItem<'a>>) -> Self {
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
}

pub struct App<'a> {
    pub todo: ToDoState<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            todo: ToDoState::new(vec![
                ToDoItem("Item 1", false),
                ToDoItem("Item 2", false),
                ToDoItem("Item 3", false),
            ]),
        }
    }
}
