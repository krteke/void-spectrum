//! Visual Ratatui adapter demo.

// use crossterm::{
//     event::{self, Event, KeyCode},
//     execute,
//     terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
// };
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Layout},
    style::Modifier,
    widgets::{Block, Borders, Paragraph},
};
use spectrum_core::Color;
use spectrum_ratatui::RatatuiStyleAdapter;
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let bg = Color::new(20, 24, 32);
    let fg = Color::new(232, 238, 247);
    let accent = Color::new(125, 92, 255);
    let muted = Color::new(142, 151, 170);

    loop {
        terminal.draw(|frame| {
            let [title, body, footer] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .areas(frame.area());

            frame.render_widget(
                Block::new().style((Some(fg), Some(bg)).style()),
                frame.area(),
            );
            frame.render_widget(
                Paragraph::new("Void Spectrum Ratatui")
                    .style(
                        (Some(accent), Some(bg))
                            .style()
                            .add_modifier(Modifier::BOLD),
                    )
                    .block(Block::new().borders(Borders::BOTTOM)),
                title,
            );
            frame.render_widget(
                Paragraph::new("Spectrum Color values are converted through adapter traits.")
                    .style((Some(fg), Some(bg)).style()),
                body,
            );
            frame.render_widget(
                Paragraph::new("Press q or Esc to exit").style((Some(muted), Some(bg)).style()),
                footer,
            );
        })?;

        if event::poll(Duration::from_millis(200))?
            && matches!(
                event::read()?,
                Event::Key(key) if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
            )
        {
            return Ok(());
        }
    }
}
