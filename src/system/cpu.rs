use sysinfo::{CpuExt, System, SystemExt};
use std::time::Duration;
use std::collections::VecDeque;

const HISTORY_SIZE: usize = 100; // Store last 100 data points

pub struct CpuState {
    system: System,
    usage_per_core: Vec<f32>,
    average_usage: f32,
    core_count: usize,
    history: VecDeque<f32>, // Add history for graphing
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
            history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu();
        
        // Reduce sleep time to prevent UI lag
        std::thread::sleep(Duration::from_millis(100));
        self.system.refresh_cpu();
        
        self.usage_per_core = self.system.cpus().iter().map(|cpu| cpu.cpu_usage().min(100.0)).collect();
        self.core_count = self.usage_per_core.len();
        
        // Avoid division by zero and ensure average is capped
        if self.core_count > 0 {
            self.average_usage = (self.usage_per_core.iter().sum::<f32>() / self.core_count as f32).min(100.0);
        } else {
            self.average_usage = 0.0;
        }

        // Add current usage to history
        self.history.push_back(self.average_usage);
        if self.history.len() > HISTORY_SIZE {
            self.history.pop_front();
        }
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
    
    // Add method to get history data for graphs
    pub fn get_history(&self) -> &VecDeque<f32> {
        &self.history
    }
}
