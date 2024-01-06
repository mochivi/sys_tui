// pub mod sys_pooler;

use std::ops::Deref;

use sysinfo::{Disks, Networks, System, CpuRefreshKind, RefreshKind};
use tui::widgets::{ListItem, List};

pub struct SysInfo {
    pub disks: Disks,
    pub networks: Networks,
    pub system: System,
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

    pub fn get_cpus_usage(&self) -> Vec<f32> {
        let mut cpu_usage_vec: Vec<f32> = Vec::new();
        for cpu in self.system.cpus().iter() {
            cpu_usage_vec.push(cpu.cpu_usage());
        }
        cpu_usage_vec
    }

    pub fn get_disk_names(&self) -> Vec<&str> {
        self.disks
            .list()
            .iter()
            .map(|d| d.name().to_str().unwrap())
            .collect::<Vec::<&str>>()
    }
}

pub fn setup() -> SysInfo {
    let state: SysInfo = SysInfo::new();
    return state;
}