// pub mod sys_pooler;

use sysinfo::{Components, Disks, Networks, System, CpuRefreshKind, RefreshKind};

pub struct SysInfo {
    pub components: Components,
    pub disks: Disks,
    pub networks: Networks,
    pub system: System,
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            components: Components::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            )
        }
    }

    // Refresh all data on a single tick
    pub fn refresh(&mut self) {
        self.refresh_components_data();
    }

    pub fn refresh_components_data(&mut self) {
        for component in &self.components {
            println!("{:?}", component);
        }
    }

}

pub fn setup() -> SysInfo {
    let state: SysInfo = SysInfo::new();
    return state;
}