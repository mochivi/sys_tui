#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::VecDeque;

use ratatui::{
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
    start_time: Instant,
    pub graph_size_percentage: u16
}

impl State {
    pub fn new(sys: sys_poller::SysInfo) -> Self {
        Self {
            system: sys,
            graph: Graph::CPU,
            cpu_dataset: CpuDataset::new(),
            start_time: Instant::now(),
            graph_size_percentage: 60
        }
    }

    pub fn refresh(&mut self) -> f64 {
        self.system.refresh();
        let elapsed_ms = self.refresh_cpu_dataset();
        elapsed_ms
    }

    pub fn refresh_cpu_dataset(&mut self) -> f64 {
        
        // Refresh cpu usage
        let elapsed_ms = self.start_time.elapsed().as_millis() as f64;
        self.cpu_dataset.update_cpu_usage(
            elapsed_ms,
            self.system.get_avg_cpu_usage().into()
        );        
        
        return elapsed_ms;
    }

    pub fn set_graph_cpu(&mut self) {
        self.graph = Graph::CPU
    }

    pub fn set_graph_memory(&mut self) {
        self.graph = Graph::MEMORY
    }

    pub fn set_graph_disk(&mut self) {
        self.graph = Graph::DISK
    }

    pub fn expand_graph_size(&mut self) {
        self.graph_size_percentage += 2;
        if self.graph_size_percentage >= 100 {
            self.graph_size_percentage = 100;
        }
    }

    pub fn reduce_graph_size(&mut self) {
        // Avoid assigning a u16 value as negative
        if self.graph_size_percentage <= 0 {
            self.graph_size_percentage = 100;
        } else {
            self.graph_size_percentage -= 2;
        }
    }
}

pub struct CpuDataset {
    pub cpu_usage: VecDeque<(f64, f64)>
}

impl CpuDataset {
    pub fn new() -> Self {
        Self {
            cpu_usage: VecDeque::with_capacity(100000),
        }
    }

    // Update vec and insert values
    pub fn update_cpu_usage(&mut self, elapsed_ms: f64, value: f64) {
        if elapsed_ms >= 25000.0 {
            self.cpu_usage.pop_front();
        }
        self.cpu_usage.push_back(
            (
                elapsed_ms,
                value
            )
        );        
    }

    pub fn get_cpu_usage_as_slice(&mut self) -> (&[(f64, f64)], &[(f64, f64)]) {
        self.cpu_usage.make_contiguous();
        self.cpu_usage.as_slices()
    }   
}

