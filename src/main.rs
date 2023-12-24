use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode};
use ratatui::style::Modifier;
use ratatui::{
    prelude::{CrosstermBackend, Style, Terminal},
    widgets::*,
};
use std::io::stderr;
fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

    loop {
        terminal.draw(|f| {
            f.render_widget(
                List::new(["Item 1", "Item 2", "Item 3"])
                    .block(Block::default().title("To-Do").borders(Borders::ALL))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .repeat_highlight_symbol(true)
                    .direction(ListDirection::TopToBottom),
                f.size(),
            )
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
