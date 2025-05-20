#[allow(dead_code)]
pub mod cpu;
#[allow(dead_code)]
pub mod memory;
#[allow(dead_code)]
pub mod process;
#[allow(dead_code)]
pub mod disk;
#[allow(dead_code)]
pub mod network;

pub use cpu::CpuState;
pub use memory::MemoryState;
pub use process::ProcessList;
pub use disk::DiskState;
pub use network::NetworkState;

pub struct SystemState {
    pub cpu: CpuState,
    pub memory: MemoryState,
    pub processes: ProcessList,
    pub disk: DiskState,
    pub network: NetworkState,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            cpu: CpuState::new(),
            memory: MemoryState::new(),
            processes: ProcessList::new(),
            disk: DiskState::new(),
            network: NetworkState::new(),
        }
    }

    pub fn update(&mut self) {
        self.cpu.update();
        self.memory.update();
        self.processes.update();
        self.disk.update();
        self.network.update();
    }
}
