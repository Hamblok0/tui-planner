use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::layout::{Constraint, Direction};
use ratatui::style::Modifier;
use ratatui::{
    prelude::{Color, CrosstermBackend, Style, Terminal, Layout},
    widgets::*,
};
use std::io::stdout;
use tui_textarea::{Input, Key};

mod app;
use crate::app::{App, Modal};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = app
                .todo
                .items
                .iter()
                .map(|i| match i.1 {
                    true => {
                        return ListItem::new(&*i.0)
                            .style(Style::default().add_modifier(Modifier::CROSSED_OUT))
                    }
                    false => return ListItem::new(&*i.0),
                })
                .collect();

            let area = f.size();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("To Do"))
                .highlight_style(Style::default().bg(Color::LightGreen))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, area, &mut app.todo.state);
            if let Modal::Active(ref textareas, _) = app.modal {
                let area = app.modal.get_center(f.size());
                let layout = Layout::new(
                    Direction::Vertical,
                    [Constraint::Percentage(10), Constraint::Percentage(90)]
                ).split(area);
                f.render_widget(Clear, area);
                f.render_widget(textareas[0].widget(), layout[0]);
                f.render_widget(textareas[1].widget(), layout[1]);
            }
        })?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match app.modal {
                    Modal::Active(ref mut textareas, which) => match key.into() {
                        Input { key: Key::Esc, .. } => {
                            app.modal.toggle();
                        }
                        Input {
                            key: Key::Enter, ..
                        } => {
                            app.todo.create_task(textareas[which].lines().join(""));
                            app.modal.toggle();
                        }
                        input => {
                            textareas[which].input(input);
                        }
                    },
                    Modal::Inactive => match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Char('c') => app.todo.toggle_complete(),
                        crossterm::event::KeyCode::Char('j') => app.todo.next(),
                        crossterm::event::KeyCode::Char('k') => app.todo.previous(),
                        crossterm::event::KeyCode::Char('d') => app.todo.delete_task(),
                        crossterm::event::KeyCode::Char('n') => app.modal.toggle(),
                        _ => {}
                    },
                }
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
