use sysinfo::{ProcessExt, System, SystemExt, Pid};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Process {
    pid: Pid,
    name: String,
    cmd: Vec<String>,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    user_id: Option<String>,
}

impl Process {
    fn new(pid: Pid, process: &sysinfo::Process) -> Self {
        Self {
            pid,
            name: process.name().to_string(),
            cmd: process.cmd().to_vec(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            status: format!("{:?}", process.status()),
            user_id: process.user_id().map(|id| id.to_string()),
        }
    }

    pub fn get_pid(&self) -> Pid {
        self.pid
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn get_memory_usage(&self) -> u64 {
        self.memory_usage
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    pub fn get_command(&self) -> &[String] {
        &self.cmd
    }
}

#[derive(Debug)]
pub struct ProcessList {
    system: System,
    processes: HashMap<Pid, Process>,
}

impl ProcessList {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_processes();
        
        let processes = system
            .processes()
            .iter()
            .map(|(pid, process)| (*pid, Process::new(*pid, process)))
            .collect();
        
        Self { system, processes }
    }

    pub fn update(&mut self) {
        self.system.refresh_processes();
        
        // Update existing processes and add new ones
        self.processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| (*pid, Process::new(*pid, process)))
            .collect();
    }

    pub fn get_process(&self, pid: &Pid) -> Option<&Process> {
        self.processes.get(pid)
    }

    pub fn get_processes(&self) -> Vec<&Process> {
        self.processes.values().collect()
    }

    pub fn get_sorted_by_cpu(&self, limit: Option<usize>) -> Vec<&Process> {
        let mut processes = self.get_processes();
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        
        if let Some(n) = limit {
            processes.truncate(n);
        }
        
        processes
    }

    pub fn get_sorted_by_memory(&self, limit: Option<usize>) -> Vec<&Process> {
        let mut processes = self.get_processes();
        processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
        
        if let Some(n) = limit {
            processes.truncate(n);
        }
        
        processes
    }

    pub fn count(&self) -> usize {
        self.processes.len()
    }
}
