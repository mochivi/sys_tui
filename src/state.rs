#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use tui::widgets::Dataset;
use std::time::Instant;
use crate::sys_pooler;


pub enum Graph {
    CPU,
    MEMORY,
    DISK
}

pub struct Datasets {
    pub cpu_usage: Vec<(f64, f64)>,
    start_time: Instant
}

impl Datasets {
    pub fn new() -> Self {
        Self {
            cpu_usage: Vec::new(),
            start_time: Instant::now()
        }
    }

    pub fn get_as_dataset(&self) -> Dataset {
        Dataset::default().data(self.cpu_usage.as_slice())
    }

    // Update vec and insert values
    pub fn update_cpu_usage(&mut self, value: f64) {
        self.cpu_usage.push(
            (
                self.start_time.elapsed().as_millis() as f64,
                value
            )
        );
    }
}

pub struct State {
    pub system: sys_pooler::SysInfo,
    pub graph: Graph,
    pub datasets: Datasets
}

impl State {
    pub fn new(sys: sys_pooler::SysInfo) -> Self {
        Self {
            system: sys,
            graph: Graph::CPU,
            datasets: Datasets::new()
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh();
        self.datasets.update_cpu_usage(self.system.get_avg_cpu_usage().into())
    }
}