// pub mod sys_pooler;

use sysinfo::{Disks, Networks, System, CpuRefreshKind, RefreshKind};
// use tui::widgets::{ListItem, List, Dataset};

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

    pub fn get_avg_cpu_usage(&self) -> f64 {
        let mut cpu_usage_vec: Vec<f64> = Vec::new();
        for cpu in self.system.cpus().iter() {
            cpu_usage_vec.push(cpu.cpu_usage().into());
        }
       return cpu_usage_vec.iter().sum::<f64>() / cpu_usage_vec.len() as f64;
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