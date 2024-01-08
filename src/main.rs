#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{io, thread, time::Duration};
use tui::{
    Terminal,
    backend::{CrosstermBackend, Backend},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
    let mut state: State = State::new(sys_poller::setup());
    loop {
        // Refresh state before next loop
        // This will add new data to datasets etc.
        state.refresh();

        // Draw data on the terminal and sleep for 10 ms
        terminal.draw(|f| ui::create_ui(f, &mut state))?;
        thread::sleep(Duration::from_millis(10));
        
        // Quit if user presses 'q'
        if let Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}

// fn main() {
//     let sys = sys_poller::SysInfo::new();

//     for cpu in sys.system.cpus() {
//         println!("{}", cpu.cpu_usage())
//     }

//     // Print disks
//     for disk in sys.disks.list() {
//         println!("{disk:?}");
//     }

//     // Print networks
//     for (interface_name, network) in &sys.networks {
//         println!("[{interface_name}]: {network:?}");
//     }
// }
