use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::style::Modifier;
use ratatui::{
    prelude::{CrosstermBackend, Style, Terminal, Color},
    widgets::*,
};
use std::io::stderr;
use tui_textarea::{Input, Key, TextArea};

mod app;
use crate::app::App;
fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut app = App::new();
    let mut textarea = TextArea::default();

    textarea.set_cursor_line_style(Style::default());
    textarea.set_placeholder_text("Short To-Do Description...");

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = app.todo.items.iter().map(|i| {
                match i.1 {
                    true => return ListItem::new(&*i.0).style(Style::default().add_modifier(Modifier::CROSSED_OUT)),
                    false => return ListItem::new(&*i.0), 
                }
            })
            .collect();  

            let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("To Do"))
            .highlight_style(Style::default().bg(Color::LightGreen))
            .highlight_symbol(">> ");
                        
            f.render_stateful_widget(list, f.size(), &mut app.todo.state)
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        crossterm::event::KeyCode::Char('c') => app.todo.toggle_complete(),
                        crossterm::event::KeyCode::Char('j') => app.todo.next(),
                        crossterm::event::KeyCode::Char('k') => app.todo.previous(),
                        crossterm::event::KeyCode::Char('d') => app.todo.delete_task(),
                        _ => {}
                    }
                }
            }
        }
    }

    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
