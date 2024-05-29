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

mod app;
mod db;
mod key_events;
mod local_data;
mod todo;

use crate::app::{App, View};
use crate::key_events::*;

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

            match app.view {
                View::Modal(ref modal) => {
                    let area = modal.get_center(f.size());
                    let layout = Layout::new(
                        Direction::Vertical,
                        [Constraint::Percentage(10), Constraint::Percentage(90)],
                    )
                    .split(area);
                    f.render_widget(Clear, area);
                    f.render_widget(modal.textareas[0].widget(), layout[0]);
                    f.render_widget(modal.textareas[1].widget(), layout[1]);
                }
                _ => (),
            }
        })?;

        if let Some(_) = key_events(&mut app) {
            break;
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
