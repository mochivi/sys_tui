#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::ffi::OsString;

use sysinfo::{Disks, Networks, System, CpuRefreshKind, RefreshKind, DiskKind};
// use tui::widgets::{ListItem, List, Dataset};

pub struct SysInfo {
    pub disks: Disks,
    pub networks: Networks,
    pub system: System,
}

pub struct DiskData {
    pub name: Box<OsString>,
    pub kind: DiskKind,
    pub file_system: Box<OsString>,
    pub total_space: u64,
    pub available_space: u64,
}

impl DiskData {
    pub fn new(name: Box<OsString>, kind: DiskKind, file_system: Box<OsString>, total_space: u64, available_space: u64) -> Self {
        Self {name, kind, file_system, total_space, available_space}
    }
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            )
        }
    }

    // Refresh all data on a single tick
    pub fn refresh(&mut self) {
        self.refresh_disks();
        self.refresh_networks();
        self.refresh_system();
    }

    pub fn refresh_disks(&mut self) {
        self.disks.refresh();
    }

    pub fn refresh_networks(&mut self) {
        self.networks.refresh_list();
        self.networks.refresh();
    }

    pub fn refresh_system(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_avg_cpu_usage(&self) -> f64 {
        let mut cpu_usage_vec: Vec<f64> = Vec::new();
        for cpu in self.system.cpus().iter() {
            cpu_usage_vec.push(cpu.cpu_usage().into());
        }
       return cpu_usage_vec.iter().sum::<f64>() / cpu_usage_vec.len() as f64;
    }

    pub fn get_disk_data(&self) -> Vec<DiskData> {
        self.disks.list().iter().map(|d| {
            DiskData::new(
                Box::new(d.name().to_owned()),
                d.kind(),
                Box::new(d.file_system().to_owned()),
                d.total_space(),
                d.available_space()
            )
        }).collect::<Vec<DiskData>>()
    }

    // pub fn get_disk_names(&self) -> Vec<&str> {
    //     self.disks
    //         .list()
    //         .iter()
    //         .map(|d| d.name().to_str().unwrap())
    //         .collect::<Vec::<&str>>()
    // }
}

pub fn setup() -> SysInfo {
    let state: SysInfo = SysInfo::new();
    return state;
}