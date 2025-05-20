use sysinfo::{DiskExt, System, SystemExt};

pub struct DiskInfo {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    file_system: String,
}

impl DiskInfo {
    fn new(disk: &sysinfo::Disk) -> Self {
        Self {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_mount_point(&self) -> &str {
        &self.mount_point
    }

    pub fn get_total_space(&self) -> u64 {
        self.total_space
    }

    pub fn get_available_space(&self) -> u64 {
        self.available_space
    }

    pub fn get_used_space(&self) -> u64 {
        self.total_space - self.available_space
    }

    pub fn get_usage_percent(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        ((self.total_space - self.available_space) as f64 / self.total_space as f64) * 100.0
    }

    pub fn get_file_system(&self) -> &str {
        &self.file_system
    }
}

pub struct DiskState {
    system: System,
    disks: Vec<DiskInfo>,
}

impl DiskState {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_disks_list();
        
        let disks = system
            .disks()
            .iter()
            .map(|disk| DiskInfo::new(disk))
            .collect();
        
        Self { system, disks }
    }

    pub fn update(&mut self) {
        self.system.refresh_disks();
        
        self.disks = self
            .system
            .disks()
            .iter()
            .map(|disk| DiskInfo::new(disk))
            .collect();
    }

    pub fn get_disks(&self) -> &[DiskInfo] {
        &self.disks
    }

    pub fn get_total_space(&self) -> u64 {
        self.disks.iter().map(|disk| disk.total_space).sum()
    }

    pub fn get_available_space(&self) -> u64 {
        self.disks.iter().map(|disk| disk.available_space).sum()
    }

    pub fn get_used_space(&self) -> u64 {
        self.get_total_space() - self.get_available_space()
    }
}
