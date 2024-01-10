#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crossterm::terminal::EnableLineWrap;
use sysinfo::DiskKind;
use ratatui::{
    Frame, 
    backend::Backend,
    widgets::{Widget, Block, Borders, Paragraph, BorderType, List, ListItem, Gauge, Dataset, Chart, Axis, GraphType, Row, Table, Wrap, LegendPosition, Padding, canvas::Label},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Color, Modifier, Style, Stylize},
    symbols::{block, Marker, border::Set},
    text::Span
};
use std::{collections::HashMap, ffi::OsString, ops::Deref, rc::Rc};
use crate::{
    state::{State, Graph},
    sys_poller::DiskData
};

const MIN_TOTAL_HEIGHT: u16 = 32;
const MIN_TOTAL_WIDTH: u16 = 40;

pub fn create_ui(f: &mut Frame, state: &mut State, elapsed_ms: f64) {
    let main_chunk: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ].as_ref()
    ).split(f.size());

    // Get all areas and their respective names as a HashMap
    let areas: HashMap<String, Rect> = separate_areas(main_chunk.deref(), &state);

    // Draw all blocks and borders etc.
    let blocks: HashMap<String, Block<'static>> = draw_blocks(f, &areas);

    // Draw actual data
    draw_description(f, &blocks.get("desc_block").unwrap().inner(*areas.get("desc_area").unwrap()));
    draw_usage(f, &blocks.get("app_usage_block").unwrap().inner(*areas.get("app_usage_area").unwrap()));
    draw_cpu(f, state, &blocks.get("cpu_block").unwrap().inner(*areas.get("cpu_info").unwrap()));
    draw_disks(
        f, 
        state,
        &blocks.get("disks_block").unwrap().inner(*areas.get("disk_info").unwrap())
    );
    match state.graph {
        Graph::CPU => {
            draw_cpu_graph(
                f,
                state,
                &blocks.get("graph_block").unwrap().inner(*areas.get("graph_area").unwrap()),
                elapsed_ms
            )
        },
        Graph::MEMORY=> {},
        Graph::DISK => {},
    }
    
    
}

// Define all areas that will containg widgets
fn separate_areas(area_arr: &[Rect], state: &State) -> HashMap<String, Rect> {

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
    let mut areas: HashMap<String, Rect> = HashMap::new();

    let uppermost_section: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref()
        )
        .split(area_arr[0]);
    areas.insert("desc_area".to_owned(), uppermost_section[0]);
    areas.insert("app_usage_area".to_owned(), uppermost_section[1]);
    
    let lower_section: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [ 
                Constraint::Percentage(100 - state.graph_size_percentage),
                Constraint::Percentage(state.graph_size_percentage)
            ].as_ref()
        )
        .split(area_arr[1]);
    areas.insert("graph_area".to_owned(), lower_section[1]);

    let info_section: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(12),
                Constraint::Percentage(25),
                Constraint::Min(8)
            ].as_ref()
        )
        .split(lower_section[0]);
    areas.insert("cpu_info".to_owned(), info_section[0]);
    areas.insert("mem_info".to_owned(), info_section[1]);
    areas.insert("disk_info".to_owned(), info_section[2]);

    areas
}

fn draw_blocks<'a>(f: &mut Frame, areas: &HashMap<String, Rect>) -> HashMap<String, Block<'a>> {
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

fn draw_description(f: &mut Frame, area: &Rect) {
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

fn draw_usage(f: &mut Frame, area: &Rect) {
    // App usage definition
    const APP_USAGE: &str = r#"
    App usage:
    Q:          Quit
    C:          Show CPU Graph
    M:          Show Memory Graph
    D:          Show Disk Graph
    A:          Expand Graph Size
    S:          Reduce Graph Size
    "#;
    let app_desc = Paragraph::new(APP_USAGE);
    f.render_widget(app_desc, *area);
}

fn draw_cpu(f: &mut Frame, state: &mut State, area: &Rect) {
    //
    // We will display the CPU brand, vendor_id
    // frequency and usage across all cores 
    //  -------------------------
    // |        CPU brand       | 
    // |------------------------|
    // |           |            |
    // |           |            |
    // |    CPU    |            |
    // | Paragraph |   VGauge   |
    // |    Info   |  (Usage %) |
    // |           |            |
    // |           |            |
    //  -------------------------

    // Separate upper and lower section
    let cpu_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(5),
                Constraint::Min(1)
            ]
        )
        .vertical_margin(1)
        .split(*area);
    let upper_section = cpu_layout[0];
    let lower_section = cpu_layout[1];

    // Draw some CPU information
    let cpu_brand = state.system.get_cpu_brand();
    let avg_frequency: f64 = state.system.get_avg_cpu_frequency() as f64 / 1000.0;
    let core_count = state.system.get_core_count();
    let processes_count = state.system.get_processes_count();

    let cpu_info = format!(
    r#"{cpu_brand}
Base speed: {avg_frequency:.2} GHz
Cores: {core_count}
Processes: {processes_count}
"#);

    let info_paragraph = Paragraph::new(cpu_info);    
    f.render_widget(info_paragraph, upper_section);

    // Draw gauge for CPU usage
    let usage = state.system.get_avg_cpu_usage();
    let label = format!("{usage:.2} %");
    
    let freq_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Thick)
                .title("Usage (%)")
                .title_alignment(Alignment::Center)
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
        )
        .percent(usage as u16)
        .label(label);
        f.render_widget(freq_gauge, lower_section);
}

fn draw_memory(f: &mut Frame, area: &Rect) {}

fn draw_disks(f: &mut Frame, state: &mut State, area: &Rect) {
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

    let disk_table = Table::new(rows, table_constraints_slice)
        .style(
            Style::default()
            .fg(Color::LightCyan)
            .bg(Color::Black)
        )
        .header(header)
        .column_spacing(0);

    // Render table
    f.render_widget(disk_table, *area);
}


fn draw_cpu_graph(f: &mut Frame, state: &mut State, area: &Rect, elapsed_ms: f64) {
    let (data, _) = state.cpu_dataset.get_cpu_usage_as_slice();
    let cpu_dataset = Dataset::default()
            .name("CPU Usage")
            .marker(Marker::HalfBlock)
            .graph_type(GraphType::Line)
            .style(
                Style::default()
                    .fg(Color::LightCyan)
                )
            .data(
                data
            );

    let mut dataset_vec = Vec::new();
    dataset_vec.push(cpu_dataset);

    let right_bound = elapsed_ms;

    // This is hardcoded and should not be.
    // Same value is used in state.rs CpuDataset update_cpu_usage()
    let left_bound: f64 = if elapsed_ms <= 25000.0 { 0.0 } else { elapsed_ms - 25000.0 };

    // Create graph labels as &[&str] based on the current right_bound f64 value.
    // This is messy and surely there's a better way I do not know of
    let mut graph_labels: Vec<f64> = Vec::new();

    if elapsed_ms <= 25000.0 {
        graph_labels.push(0.0);
    } else {
        graph_labels.push(elapsed_ms - 25000.0)
    };
    graph_labels.push(right_bound/2.0);
    graph_labels.push(right_bound);
    
    
    let graph_labels = graph_labels.into_iter().map(|i| {
        i.to_string()
    }).collect::<Vec<String>>();
    let graph_labels: Vec<&str> = graph_labels.iter().map(|i| {
        i.as_ref()
    }).collect::<Vec<&str>>();

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
                .bounds([left_bound, right_bound])
                .labels(graph_labels.as_slice().iter().cloned().map(Span::from).collect())
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
        )
        .fg(Color::White)
        .bg(Color::Black)
        .legend_position(Some(LegendPosition::TopRight));
    f.render_widget(cpu_chart, *area);

}