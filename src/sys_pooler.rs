// pub mod sys_pooler;

use sysinfo::{Disks, Networks, System, CpuRefreshKind, RefreshKind};
use tui::widgets::ListItem;

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

    pub fn get_disk_as_list_items(&self) -> Vec<ListItem> {
        let mut disk_items: Vec<ListItem> = Vec::new(); // Will hold the disk strings as ListItem
        let mut disk_string: String; // Will create the disk string for each Disk
        
        // Iterate over each Disk
        for disk in self.disks.list() {
            

            // Create disk string with disk data
            disk_string = String::from("Name: ");        
            if let Some(disk_name) = disk.name().to_str() {
                disk_string.push_str(disk_name);
                disk_string.push_str("\n");
            }
            
            // Push this disk string into a ListItem Vec
            disk_items.push(ListItem::new(disk_string.clone()));
        }
        return disk_items;
    }
}

pub fn setup() -> SysInfo {
    let state: SysInfo = SysInfo::new();
    return state;
}