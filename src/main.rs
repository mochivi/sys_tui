#![allow(unused_imports)]
#![allow(dead_code)]

use std::{io, thread, time::Duration};
use tui::{
    Frame, 
    Terminal,
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders, Paragraph, BorderType, List, ListItem, Gauge},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Color, Modifier, Style},
    symbols::block
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, style::Stylize,
};
use std::collections::HashMap;

mod sys_pooler;


// App usage definition
const APP_KEYS_DESC: &str = r#"
    App usage:
    Q:           Quit
    S:           Search
    Insert Btn:  Insert new Password
    Tab:         Go to next field
    Shift+Tab:   Go to previous filed
    Esc:         Exit insert mode
    "#;

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

// fn main() {
//     let sys = SysInfo::new();

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
    let mut sys: sys_pooler::SysInfo = sys_pooler::setup();
    loop {
        // Refresh sys before next loop
        sys.refresh();

        // Draw data on the terminal and sleep for 10 ms
        terminal.draw(|f| ui(f, &mut sys))?;
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

fn ui<B: Backend>(f: &mut Frame<B>, sys: &mut sys_pooler::SysInfo) {
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ].as_ref()
    ).split(f.size());

    // Get all areas and their respective names as a HashMap
    let mut areas: HashMap<String, Rect> = HashMap::new();
    separate_areas(&mut areas, &main_chunk);

    // Draw all blocks and borders etc.
    let blocks: HashMap<String, Block<'static>> = draw_blocks(f, &areas);

    // Draw actual data
    draw_usage(
        f,
        &blocks.get("app_usage_block").unwrap().inner(*areas.get("app_usage_area").unwrap()),
    );
    draw_disks(
        f, 
        sys,
        &blocks.get("disks_block").unwrap().inner(*areas.get("disk_info").unwrap()),
        // &blocks.get("disks_block").unwrap()
    );
    draw_cpu_graph(
        f,
        sys,
        &blocks.get("graph_block").unwrap().inner(*areas.get("graph_area").unwrap())
    );
}   

// Define all areas that will containg widgets
fn separate_areas(areas: &mut HashMap<String, Rect>, area: &Vec<Rect>) {

    // The idea is that the app looks like:
    // ------------------------------
    // |  Desc + usage              | <-- uppermost section
    // |----------------------------|
    // | some info|     graph       |
    // |          |                 |
    // |----------|  (which graph   | <-- graph section
    // | more     |   is selected   |
    // | info     |   by the user)  |
    // ------------------------------

    let uppermost_section: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref()
        )
        .split(area[0]);
    areas.insert("desc_area".to_owned(), uppermost_section[0]);
    areas.insert("app_usage_area".to_owned(), uppermost_section[1]);
    
    let lower_section: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60)
            ].as_ref()
        )
        .split(area[1]);
    areas.insert("graph_area".to_owned(), lower_section[1]);

    let info_section: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(34)
            ].as_ref()
        )
        .split(lower_section[0]);
    areas.insert("cpu_info".to_owned(), info_section[0]);
    areas.insert("mem_info".to_owned(), info_section[1]);
    areas.insert("disk_info".to_owned(), info_section[2]);
}


fn draw_blocks<'a, B: Backend>(f: &mut Frame<B>, areas: &HashMap<String, Rect>) -> HashMap<String, Block<'a>> {
    let mut blocks: HashMap<String, Block> = HashMap::new();
    let description_block = Block::default()
        .title("App Description")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
        )
        .border_type(BorderType::Plain);
    blocks.insert("desc_block".to_string(), description_block.clone());
    f.render_widget(description_block, *areas.get("desc_area").unwrap());
    

    let app_usage_block = Block::default()
        .title("App Usage")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
        )
        .border_type(BorderType::Rounded);
    blocks.insert("app_usage_block".to_string(), app_usage_block.clone());
    f.render_widget(app_usage_block, *areas.get("app_usage_area").unwrap());
    

    let cpu_block = Block::default()
        .title("CPU Information")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
        )
        .border_type(BorderType::Rounded);
    blocks.insert("cpu_block".to_string(), cpu_block.clone());
    f.render_widget(cpu_block, *areas.get("cpu_info").unwrap());
    

    let mem_block = Block::default()
        .title("Memory Information")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
        )
        .border_type(BorderType::Rounded);
    blocks.insert("mem_block".to_string(), mem_block.clone());
    f.render_widget(mem_block, *areas.get("mem_info").unwrap());
    

    let disks_block = Block::default()
        .title("Disks Information")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
        )
        .border_type(BorderType::Rounded);
    blocks.insert("disks_block".to_string(), disks_block.clone());
    f.render_widget(disks_block, *areas.get("disk_info").unwrap());
    

    let graph_block = Block::default()
        .title("Graph")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::LightCyan)
                .bg(Color::Black)
        )
        .border_type(BorderType::Double);
    blocks.insert("graph_block".to_string(), graph_block.clone());
    f.render_widget(graph_block, *areas.get("graph_area").unwrap());

    blocks
}


fn draw_usage<B: Backend>(f: &mut Frame<B>, area: &Rect) {
    let app_desc = Paragraph::new(APP_KEYS_DESC);
    f.render_widget(app_desc, *area);
}

fn draw_disks<B: Backend>(f: &mut Frame<B>, sys: &mut sys_pooler::SysInfo, area: &Rect) {
    let disk_items: Vec<&str> = sys.get_disk_names();
    
    let disk_list_items: Vec<ListItem> = disk_items
        .into_iter()
        .map(|s| ListItem::new(s))
        .collect();

    let disk_list = List::new(disk_list_items)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_widget(disk_list, *area);
}

fn draw_cpu_graph<B: Backend>(f: &mut Frame<B>, sys: &mut sys_pooler::SysInfo, area: &Rect) {
    let cpus_usage: Vec<f32> = sys.get_cpus_usage();
    // let average_usage: f32 = cpus_usage.iter().sum::<f32>() / cpus_usage.len() as f32;

    
    
    let mut cpu_constraints: Vec<Constraint> = Vec::new();
    let mut cpu_gauges = Vec::new();
    for cpu_usage in cpus_usage {
        cpu_gauges.push(
            Gauge::default()
                .gauge_style(Style::default()
                    .fg(Color::LightBlue)
                    .bg(Color::Black)
                )
                .percent(cpu_usage as u16)
        );
    }

    let cpu_graph_layout: Vec<Rect> = Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [].as_ref()
        )
        .split(*area);

    for section in cpu_graph_layout {
        f.render_widget(widget, area)
    }


    // let cpu_graph = Gauge::default()
    //     .gauge_style(Style::default()
    //         .fg(Color::LightBlue)
    //         .bg(Color::Black)
    //     )
    //     .percent(average_usage as u16);
    // f.render_widget(cpu_graph, *area);

    // let cpu_info = List::new(cpus_usage_items);
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