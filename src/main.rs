#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{io, thread, time::Duration};
use ratatui::{
    Terminal,
    backend::{CrosstermBackend, Backend},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use state::Graph;

mod sys_poller;
mod state;
mod ui;

use crate::state::State;


fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // mutable reference of terminal to run_app
    let result = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // If app executed into error
    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state: State = State::new(sys_poller::setup());
    let mut elapsed_ms: f64 = state.refresh();
    loop {
        // Draw data on the terminal and sleep for 10 ms
        terminal.draw(|f| ui::create_ui(f, &mut state, elapsed_ms))?;
        // thread::sleep(Duration::from_millis(490));

        // Refresh state before next loop
        // This will add new data to datasets etc.
        elapsed_ms = state.refresh();
        
        // Quit if user presses 'q'
        match event::poll(Duration::from_millis(50))? {
            true => {
                if let Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') => state.set_graph_cpu(),
                        KeyCode::Char('m') => state.set_graph_memory(),
                        KeyCode::Char('d') => state.set_graph_disk(),
                        KeyCode::Char('a') => state.expand_graph_size(),
                        KeyCode::Char('s') => state.reduce_graph_size(),
                        _ => {}
                    }
                }
            }
            false => continue
        }
    }
    Ok(())
}