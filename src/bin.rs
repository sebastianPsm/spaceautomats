mod ui;
mod app;
use std::{error::Error, io, time::Duration};
use argparse::{ArgumentParser, Store, StoreTrue};
use ratatui::{Terminal, backend::{Backend, CrosstermBackend}, crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, poll}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}}};
use crate::{ui::ui, app::App, app::Arguments};



pub fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = Arguments { 
        verbose: false,
        ui: false,
        ego: String::new(),
        dir: String::new(),
        x: 2000000,
        y: 2000000,
        seed: 1
    };

    /*
     * arguments
     */
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("speceautomats - A programming game inspired by droidbattles");
        ap.refer(&mut arguments.verbose).add_option(&["-v", "--verbose"], StoreTrue, "Start in verbose mode");
        ap.refer(&mut arguments.ui).add_option(&["-u", "--ui"], StoreTrue, "Start user interface");
        ap.refer(&mut arguments.ego).add_option(&["-e", "--ego"], Store, "Script of the ego spaceautomat (lua file)");
        ap.refer(&mut arguments.dir).add_option(&["-d", "--dir"], Store, "Directory of speceautomats (directory with lua files)");
        ap.refer(&mut arguments.x).add_option(&["-x"], Store, "Width of the arena (default: 5000)");
        ap.refer(&mut arguments.y).add_option(&["-y"], Store, "Height of the arena  (default: 5000)");
        ap.refer(&mut arguments.seed).add_option(&["-s", "--seed"], Store, "Pseudo randum number generator seed  (default: 123456789)");
        ap.parse_args_or_exit();
    }

    if arguments.verbose {
        println!("Arguments:");
        println!("- verbose: yes");
        println!("- ui: {}", if arguments.ui { "yes" } else { "no" });
        println!("- ego: '{}'", arguments.ego);
        println!("- dir: '{}'", arguments.dir);
        println!("- x: {}", arguments.x);
        println!("- y: {}", arguments.y);
        println!("- seed: {}", arguments.seed);
    }

    let mut app = App::new(arguments);

    app.init();

    if app.is_ui() {
        /*
        * Ui start/stop
        */
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stderr);
        let mut terminal = Terminal::new(backend)?;

            enable_raw_mode()?;
            run_app(&mut terminal, &mut app)?;
            disable_raw_mode()?;

        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    }
    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> where io::Error: From<B::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Result::Ok(true) = poll(Duration::from_millis(1)) {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue; // Skip events that are not KeyEventKind::Press
                }

                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Up => {
                        app.selected_automat = app.selected_automat.saturating_sub(1)
                    }
                    KeyCode::Down => {
                        if app.selected_automat < app.sim.count_automats().saturating_sub(1) {
                            app.selected_automat = app.selected_automat.saturating_add(1)
                        }
                    }
                    _ => {}
                }
            }
        }
        app.sim.step();
    }
}