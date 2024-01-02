use std::{io, thread, time::Duration};
use tui::{
    Frame, 
    Terminal,
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, BorderType},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Color, Modifier, Style}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


mod sys_pooler;

const APP_KEYS_DESC: &str = r#"
    App usage:
    Q:           Quit
    S:           Search
    Insert Btn:  Insert new Password
    Tab:         Go to next field
    Shift+Tab:   Go to previous filed
    Esc:         Exit insert mode
    "#;
// App state definition

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
    let mut state: sys_pooler::SysInfo = sys_pooler::setup();
    loop {
        // Refresh state before next loop
        state.refresh();

        // Draw data on the terminal and sleep for 10 ms
        terminal.draw(|f| ui(f, &mut state))?;
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

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut sys_pooler::SysInfo) {
    let main_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)
        ].as_ref()
        ).split(f.size());

    // Draw app desciption into top left
    let description_block = Block::default()
        .title("App usage")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);
    f.render_widget(description_block, main_chunk[0]);
    draw_description(f, main_chunk[0]);
    
    // Render components
    let components_block = Block::default()
        .title("System Information")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);
    f.render_widget(components_block, main_chunk[1]);
    draw_components(f, state, main_chunk[1]);
}

fn draw_description<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let new_section_chunk = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60)
                // Constraint::Min(4),
                // Constraint::Length(3),
                // Constraint::Length(3),
                // Constraint::Length(3),
                // Constraint::Length(3),
            ].as_ref()
        )
        .split(area);
    let app_desc = Paragraph::new(APP_KEYS_DESC);
    f.render_widget(app_desc, new_section_chunk[0]);
}

fn draw_components<B: Backend>(f: &mut Frame<B>, state: &mut sys_pooler::SysInfo, area: Rect) {
    let new_section_chunk = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75)
                // Constraint::Min(4),
                // Constraint::Length(3),
                // Constraint::Length(3),
                // Constraint::Length(3),
                // Constraint::Length(3),
            ].as_ref()
        )
        .split(area);

    
    
    for component in &state.components {
        println!("{}", component.label());
        let desc = Paragraph::new(component.label());
        f.render_widget(desc, new_section_chunk[0]);
    }
}

//     let title_input = Paragraph::new(state.new_title.to_owned())
//         .block(Block::default().title("Title").borders(Borders::ALL).border_type(BorderType::Rounded))
//         .style(match state.mode {
//             InputMode::Title => Style::default().fg(Color::Yellow),
//             _ => Style::default()
//         });
//     f.render_widget(title_input, new_section_chunk[1]);

//     let username_input = Paragraph::new(state.new_username.to_owned())
//         .block(Block::default().title("Username").borders(Borders::ALL).border_type(BorderType::Rounded))
//         .style(match state.mode {
//             InputMode::Username => Style::default().fg(Color::Yellow),
//             _ => Style::default()
//         });
//     f.render_widget(username_input, new_section_chunk[2]);

//     let password_input = Paragraph::new(state.new_password.to_owned())
//         .block(Block::default().title("Password").borders(Borders::ALL).border_type(BorderType::Rounded))
//         .style(match state.mode {
//             InputMode::Password => Style::default().fg(Color::Yellow),
//             _ => Style::default()
//         });
//     f.render_widget(password_input, new_section_chunk[3]);

//     let submit_btn = Paragraph::new("Submit")
//         .alignment(Alignment::Center)
//         .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
//         .style(match state.mode {
//             InputMode::Submit => Style::default().fg(Color::Yellow),
//             _ => Style::default()
//         });
//     f.render_widget(submit_btn, new_section_chunk[4]);
// }