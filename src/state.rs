#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use tui::{
    widgets::{Dataset, GraphType},
    style::{Style, Color}
};
use std::time::Instant;
use crate::sys_poller;


pub enum Graph {
    CPU,
    MEMORY,
    DISK
}

pub struct State {
    pub system: sys_poller::SysInfo,
    pub graph: Graph,
    pub cpu_dataset: CpuDataset,
    start_time: Instant
}

pub struct CpuDataset {
    pub cpu_usage: Vec<(f64, f64)>,
}

impl State {
    pub fn new(sys: sys_poller::SysInfo) -> Self {
        Self {
            system: sys,
            graph: Graph::CPU,
            cpu_dataset: CpuDataset::new(),
            start_time: Instant::now()
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh();
        self.cpu_dataset.update_cpu_usage(
            self.start_time.elapsed().as_millis() as f64,
            self.system.get_avg_cpu_usage().into()
        );
    }
}

impl CpuDataset {
    pub fn new() -> Self {
        Self {
            cpu_usage: Vec::new()
        }
    }

    // Update vec and insert values
    pub fn update_cpu_usage(&mut self, time_ms: f64, value: f64) {
        self.cpu_usage.push(
            (
                time_ms,
                value
            )
        );
    }

    pub fn get_dataset(&mut self) -> Dataset {
        Dataset::default()
            .name("CPU Usage")
            .marker(tui::symbols::Marker::Dot)
            .graph_type(GraphType::Line)
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                )
            .data(self.cpu_usage.as_slice())
    }
}

