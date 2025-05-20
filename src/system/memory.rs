use std::collections::VecDeque;
use sysinfo::{System, SystemExt};

const HISTORY_SIZE: usize = 100;

pub struct MemoryState {
    system: System,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    memory_history: VecDeque<f64>,
    swap_history: VecDeque<f64>,
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
            memory_history: VecDeque::with_capacity(HISTORY_SIZE),
            swap_history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_memory();
        self.total_memory = self.system.total_memory();
        self.used_memory = self.system.used_memory();
        self.total_swap = self.system.total_swap();
        self.used_swap = self.system.used_swap();

        let mem_percent = self.get_memory_usage_percent();
        let swap_percent = self.get_swap_usage_percent();

        self.memory_history.push_back(mem_percent);
        self.swap_history.push_back(swap_percent);

        if self.memory_history.len() > HISTORY_SIZE {
            self.memory_history.pop_front();
        }

        if self.swap_history.len() > HISTORY_SIZE {
            self.swap_history.pop_front();
        }
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

    pub fn get_memory_history(&self) -> &VecDeque<f64> {
        &self.memory_history
    }

    pub fn get_swap_history(&self) -> &VecDeque<f64> {
        &self.swap_history
    }
}
