#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use sysinfo::DiskKind;
use tui::{
    Frame, 
    backend::Backend,
    widgets::{Widget, Block, Borders, Paragraph, BorderType, List, ListItem, Gauge, Dataset, Chart, Axis, GraphType, Row, Table, Wrap},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Color, Modifier, Style},
    symbols::block, text::Span
};
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, style::Stylize,
// };
use std::{collections::HashMap, ffi::OsString};
use crate::{
    state::{State, Graph},
    sys_poller::DiskData
};

pub fn create_ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ].as_ref()
    ).split(f.size());

    // Get all areas and their respective names as a HashMap
    let mut areas: HashMap<String, Rect> = HashMap::new();
    separate_areas(&mut areas, &main_chunk);

    // Draw all blocks and borders etc.
    let blocks: HashMap<String, Block<'static>> = draw_blocks(f, &areas);

    // Draw actual data
    draw_description(f, &blocks.get("desc_block").unwrap().inner(*areas.get("desc_area").unwrap()));
    draw_usage(f, &blocks.get("app_usage_block").unwrap().inner(*areas.get("app_usage_area").unwrap()));
    draw_disks(
        f, 
        state,
        &blocks.get("disks_block").unwrap().inner(*areas.get("disk_info").unwrap())
    );
    draw_cpu_graph(
        f,
        state,
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
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref()
        )
        .split(area[0]);
    areas.insert("desc_area".to_owned(), uppermost_section[0]);
    areas.insert("app_usage_area".to_owned(), uppermost_section[1]);
    
    let lower_section: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref()
        )
        .split(area[1]);
    areas.insert("graph_area".to_owned(), lower_section[1]);

    let info_section: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20)
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

fn draw_description<B: Backend>(f: &mut Frame<B>, area: &Rect) {
    // App description
    const APP_DESCRIPTION: &str = r#"
    This app allows the user to monitor CPU usage, memory and disks.
    Monitoring can be done by switching the current graph seen on the right.
    "#;
    let app_desc = Paragraph::new(APP_DESCRIPTION)
        .wrap(Wrap {trim: true})
        .alignment(Alignment::Left)
        .style(
            Style::default()
            .fg(Color::Green)
            .bg(Color::Black)
        );
    f.render_widget(app_desc, *area);
}


fn draw_usage<B: Backend>(f: &mut Frame<B>, area: &Rect) {
    // App usage definition
    const APP_USAGE: &str = r#"
    App usage:
    Q:           Quit
    S:           Search
    Insert Btn:  Insert new Password
    Tab:         Go to next field
    Shift+Tab:   Go to previous filed
    Esc:         Exit insert mode
    "#;
    let app_desc = Paragraph::new(APP_USAGE);
    f.render_widget(app_desc, *area);
}

fn draw_disks<B: Backend>(f: &mut Frame<B>, state: &mut State, area: &Rect) {
    let disks_data: Vec<DiskData> = state.system.get_disk_data();
    let mut rows: Vec<Row> = Vec::new();

    // Push disk names into rows
    let mut disk_names: Vec<String> = vec!["Name".to_string()];
    disk_names.extend(
        disks_data.iter().map(|d| {
            d.name.to_str().unwrap().to_string()
        }).collect::<Vec<String>>()
    );
    rows.push(
        Row::new(disk_names)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)
    ));

    // Push disk kinds as strings
    let mut disk_kinds: Vec<String> = vec!["Kind".to_string()];
    disk_kinds.extend(
        disks_data.iter().map(|d| {
            d.kind.to_string()
        }).collect::<Vec<String>>()    
    );
    rows.push(
        Row::new(disk_kinds)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)
    ));

    // Push disk available spaces
    let mut disk_mount_point: Vec<String> = vec!["Mount Point".to_string()];
    disk_mount_point.extend(
        disks_data.iter().map(|d| {
            d.mount_point.to_str().unwrap().to_string()
        }).collect::<Vec<String>>()
    );
    rows.push(
        Row::new(disk_mount_point)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)    
    ));

    // Push disk file system as strings
    let mut disk_file_systems: Vec<String> = vec!["File System".to_string()];
    disk_file_systems.extend(
        disks_data.iter().map(|d| {
            d.file_system.to_str().unwrap().to_string()
        }).collect::<Vec<String>>()
    );
    rows.push(
        Row::new(disk_file_systems)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)    
    ));


    // Push disk total spaces
    let mut disk_total_spaces: Vec<String> = vec!["Total Space (GB)".to_string()];
    disk_total_spaces.extend(
        disks_data.iter().map(|d| {
            let total_space_mb = d.total_space as f64 / 1_000_000_000.0;
            format!("{total_space_mb:.3}")
        }).collect::<Vec<String>>()
    );
    rows.push(
        Row::new(disk_total_spaces)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)    
    ));

    // Push disk available spaces
    let mut disk_available_spaces: Vec<String> = vec!["Available Space (GB)".to_string()];
    disk_available_spaces.extend(
        disks_data.iter().map(|d| {
            let available_space_mb = d.available_space as f64 / 1_000_000_000.0;
            format!("{available_space_mb:.3}")
        }).collect::<Vec<String>>()
    );
    rows.push(
        Row::new(disk_available_spaces)
        .style(Style::default()
            .fg(Color::White)
            .bg(Color::Black)    
    ));
    
    // Depending on the number of disks installed, adjust the column size + 1 for descriptions
    let mut table_constraints_vec: Vec<Constraint> = vec![Constraint::Min(21)];
    for _i in 0..disks_data.len() {
        // table_constraints_vec.push(Constraint::Percentage((100 / disks_data.len()) as u16))
        table_constraints_vec.push(Constraint::Min(15));
    }
    let table_constraints_slice: &[Constraint] = table_constraints_vec.as_slice();

    // Define header
    let mut header_titles: Vec<String> = vec!["".to_string()];
    for i in 0..disks_data.len() {
        header_titles.push(format!("Disk {}", i+1));
    }
    let header: Row = Row::new(header_titles);

    let disk_table = Table::new(rows)
        .style(
            Style::default()
            .fg(Color::LightCyan)
            .bg(Color::Black)
        )
        .block(Block::default())
        .header(header)
        .widths(table_constraints_slice)
        .column_spacing(0);

    // Render table
    f.render_widget(disk_table, *area);
}

fn draw_cpu_graph<B: Backend>(f: &mut Frame<B>, state: &mut State, area: &Rect) {
    let cpu_dataset = Dataset::default()
            .name("CPU Usage")
            .marker(tui::symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                )
            .data(
                state.cpu_dataset.get_cpu_usage_as_slice()
            );

    let mut dataset_vec = Vec::new();
    dataset_vec.push(cpu_dataset);

    let cpu_chart = Chart::new(dataset_vec)
        .block(Block::default())
        .x_axis(
            Axis::default()
                .title(Span::styled(
                    "Time (ms)",
                    Style::default()
                        .bg(Color::Black)
                        .fg(Color::White)
                    )
                )
                .style(
                    Style::default()
                        .bg(Color::Black)
                        .fg(Color::White)
                )
                .bounds([0.0, 50000.0])
                .labels(["0.0", "25000.0", "50000.0"].iter().cloned().map(Span::from).collect())
        )
        .y_axis(
            Axis::default()
                .title(Span::styled(
                    "Usage (%)",
                    Style::default()
                        .bg(Color::Black)
                        .fg(Color::White)
                    )
                )
                .style(
                    Style::default()
                        .bg(Color::Black)
                        .fg(Color::White)
                )
                .bounds([0.0, 100.0])
                .labels(["0.0", "25.0", "50.0", "75.0", "100.0"].iter().cloned().map(Span::from).collect())
        );
    f.render_widget(cpu_chart, *area);
    

    // let dataset = Dataset::default()
    //     .name("CPU Usage")
    //     .data(data)




    // Create layout for CPU graphs, one gauge for each CPU
    // Create constraints
    // let mut cpu_layout_constraints_vec = Vec::new();
    // for _i in 0..cpus_usage.len() {
    //     cpu_layout_constraints_vec.push(Constraint::Percentage((cpus_usage.len() / 100) as u16))
    // }
    // let cpu_layout_constraints_slice = cpu_layout_constraints_vec.as_slice();   
    
    // // Create Layout
    // let cpu_graph_layout: Vec<Rect> = Layout::default()
    //     .margin(1)
    //     .direction(Direction::Vertical)
    //     .constraints(
    //         cpu_layout_constraints_slice.as_ref()
    //     )
    //     .split(*area);

    // // Render Gauges
    // let mut cpu_gauge: Gauge;
    // for (index, cpu_usage) in cpus_usage.into_iter().enumerate() {
    //     cpu_gauge = Gauge::default()
    //         .gauge_style(Style::default()
    //             .fg(Color::LightBlue)
    //             .bg(Color::Black)
    //         )
    //         .percent(cpu_usage as u16);
    //     f.render_widget(cpu_gauge, *cpu_graph_layout.get(index).unwrap());
    // }

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