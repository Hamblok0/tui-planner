use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::layout::{Constraint, Direction};
use ratatui::style::Modifier;
use ratatui::{
    prelude::{Color, CrosstermBackend, Layout, Style, Terminal},
    widgets::*,
};
use std::io::stdout;
use tui_textarea::{Input, Key};

mod app;
use crate::app::{App, Modal, ModalType};

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
                .map(|i| match i.complete {
                    true => {
                        return ListItem::new(&*i.title)
                            .style(Style::default().add_modifier(Modifier::CROSSED_OUT))
                    }
                    false => return ListItem::new(&*i.title),
                })
                .collect();

            let area = f.size();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("To Do"))
                .highlight_style(Style::default().bg(Color::LightGreen))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, area, &mut app.todo.state);
            if let Modal::New(ref textareas, _) = app.modal {
                let area = app.modal.get_center(f.size());
                let layout = Layout::new(
                    Direction::Vertical,
                    [Constraint::Percentage(10), Constraint::Percentage(90)],
                )
                .split(area);
                f.render_widget(Clear, area);
                f.render_widget(textareas[0].widget(), layout[0]);
                f.render_widget(textareas[1].widget(), layout[1]);
            }
        })?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match app.modal {
                    Modal::New(ref mut textareas, which) => match key.into() {
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
                    Modal::Inactive => match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Char('c') => app.todo.toggle_complete(),
                        crossterm::event::KeyCode::Char('j') => app.todo.next(),
                        crossterm::event::KeyCode::Char('k') => app.todo.previous(),
                        crossterm::event::KeyCode::Char('d') => app.todo.delete_task(),
                        crossterm::event::KeyCode::Char('n') => app.toggle_modal(ModalType::New),
                        crossterm::event::KeyCode::Char('v') => app.toggle_modal(ModalType::View),
                        _ => {}
                    },
                    _ => (),
                }
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
