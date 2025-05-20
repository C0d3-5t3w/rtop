use sysinfo::{System, SystemExt};

pub struct MemoryState {
    system: System,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

impl MemoryState {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();
        
        Self {
            total_memory: system.total_memory(),
            used_memory: system.used_memory(),
            total_swap: system.total_swap(),
            used_swap: system.used_swap(),
            system,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_memory();
        self.total_memory = self.system.total_memory();
        self.used_memory = self.system.used_memory();
        self.total_swap = self.system.total_swap();
        self.used_swap = self.system.used_swap();
    }

    pub fn get_total_memory(&self) -> u64 {
        self.total_memory
    }

    pub fn get_used_memory(&self) -> u64 {
        self.used_memory
    }

    pub fn get_memory_usage_percent(&self) -> f64 {
        if self.total_memory == 0 {
            return 0.0;
        }
        (self.used_memory as f64 / self.total_memory as f64) * 100.0
    }

    pub fn get_total_swap(&self) -> u64 {
        self.total_swap
    }

    pub fn get_used_swap(&self) -> u64 {
        self.used_swap
    }

    pub fn get_swap_usage_percent(&self) -> f64 {
        if self.total_swap == 0 {
            return 0.0;
        }
        (self.used_swap as f64 / self.total_swap as f64) * 100.0
    }
}
