#[allow(dead_code)]
mod util;
mod event;
mod demo;

use std::error::Error;
use std::time::Duration;
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use argh::FromArgs;
use crate::{
    demo::{ui, App},
    event::{Config, Event, Events},
};

#[argh(description = "making it compile lul")]
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(cli.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();
    let mut app = App::new("Termion demo", cli.enhanced_graphics);
    
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Up => {
                    app.on_up();
                }
                Key::Down => {
                    app.on_down();
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}