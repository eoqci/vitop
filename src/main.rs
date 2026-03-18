use std::{
    io::{self},
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{app::app::App, ui::draw};

pub mod app;
pub mod collector;
pub mod error;
pub mod types;
pub mod ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    //open a sub terminal
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Start app state
    let mut app = App::new();

    //tick-rate
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();

    app.update_data();
    // main loop
    while !app.should_quit {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        // Poll event with timout - sleep thread, no busy-wait
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        _ => {}
                    }
                }
            }
        }
        //Only update + render when enough 1s
        if last_tick.elapsed() >= tick_rate {
            app.update_data();
            terminal.draw(|f| draw::draw_ui(f, &mut app))?;
            last_tick = Instant::now();
        }
    }

    // clean before quit
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
