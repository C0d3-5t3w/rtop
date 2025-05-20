use sysinfo::{CpuExt, System, SystemExt};
use std::time::Duration;

pub struct CpuState {
    system: System,
    usage_per_core: Vec<f32>,
    average_usage: f32,
    core_count: usize,
}

impl CpuState {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu();
        
        Self {
            system,
            usage_per_core: Vec::new(),
            average_usage: 0.0,
            core_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu();
        
        // Wait a bit for accurate CPU measurements
        std::thread::sleep(Duration::from_millis(250));
        self.system.refresh_cpu();
        
        self.usage_per_core = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        self.core_count = self.usage_per_core.len();
        self.average_usage = self.usage_per_core.iter().sum::<f32>() / self.core_count as f32;
    }

    pub fn get_average_usage(&self) -> f32 {
        self.average_usage
    }

    pub fn get_core_usage(&self, core_idx: usize) -> Option<f32> {
        self.usage_per_core.get(core_idx).copied()
    }

    pub fn get_core_count(&self) -> usize {
        self.core_count
    }
}
