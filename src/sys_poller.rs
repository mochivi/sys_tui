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
    pub mount_point: Box<OsString>
}

impl DiskData {
    pub fn new(
        name: Box<OsString>,
        kind: DiskKind,
        file_system: Box<OsString>, 
        total_space: u64,
        available_space: u64,
        mount_point: Box<OsString>
    ) -> Self {
        Self {name, kind, file_system, total_space, available_space, mount_point}
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
        return self.system.global_cpu_info().cpu_usage() as f64;
    }

    pub fn get_avg_cpu_frequency(&self) -> u64 {
        let mut cpu_freq_vec: Vec<u64> = Vec::new();
        for cpu in self.system.cpus().iter() {
            cpu_freq_vec.push(cpu.frequency())
        } 
        return cpu_freq_vec.iter().sum::<u64>() / cpu_freq_vec.len() as u64;
    }

    pub fn get_core_count(&self) -> usize {
        self.system.physical_core_count().unwrap()
    }

    pub fn get_cpu_brand(&self) -> String {
        self.system.cpus()[0].brand().to_string()
    }

    pub fn get_processes_count(&self) -> usize{
        self.system.processes().len()
    }

    pub fn get_disk_data(&self) -> Vec<DiskData> {
        self.disks.list().iter().map(|d| {
            DiskData::new(
                Box::new(d.name().to_owned()),
                d.kind(),
                Box::new(d.file_system().to_owned()),
                d.total_space(),
                d.available_space(),
                Box::new(d.mount_point().as_os_str().to_owned())
            )
        }).collect::<Vec<DiskData>>()
    }
}

pub fn setup() -> SysInfo {
    let state: SysInfo = SysInfo::new();
    return state;
}