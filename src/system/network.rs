use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use sysinfo::{NetworkExt, System, SystemExt};

const HISTORY_SIZE: usize = 100;

pub struct NetworkInterface {
    name: String,
    received_bytes: u64,
    transmitted_bytes: u64,
    received_packets: u64,
    transmitted_packets: u64,

    prev_received_bytes: u64,
    prev_transmitted_bytes: u64,
    receive_rate: f64,
    transmit_rate: f64,
    last_update: Instant,
    receive_rate_history: VecDeque<f64>,
    transmit_rate_history: VecDeque<f64>,
}

impl NetworkInterface {
    fn new(name: &str, network: &sysinfo::NetworkData) -> Self {
        Self {
            name: name.to_string(),
            received_bytes: network.received(),
            transmitted_bytes: network.transmitted(),
            received_packets: network.packets_received(),
            transmitted_packets: network.packets_transmitted(),
            prev_received_bytes: network.received(),
            prev_transmitted_bytes: network.transmitted(),
            receive_rate: 0.0,
            transmit_rate: 0.0,
            last_update: Instant::now(),
            receive_rate_history: VecDeque::with_capacity(HISTORY_SIZE),
            transmit_rate_history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    fn update(&mut self, network: &sysinfo::NetworkData) {
        let now = Instant::now();
        let time_delta = now.duration_since(self.last_update).as_secs_f64();

        self.prev_received_bytes = self.received_bytes;
        self.prev_transmitted_bytes = self.transmitted_bytes;

        self.received_bytes = network.received();
        self.transmitted_bytes = network.transmitted();
        self.received_packets = network.packets_received();
        self.transmitted_packets = network.packets_transmitted();

        if time_delta > 0.001 {
            let rx_diff = self.received_bytes.saturating_sub(self.prev_received_bytes);
            let tx_diff = self
                .transmitted_bytes
                .saturating_sub(self.prev_transmitted_bytes);

            self.receive_rate = (rx_diff as f64 / time_delta).min(f64::MAX / 2.0);
            self.transmit_rate = (tx_diff as f64 / time_delta).min(f64::MAX / 2.0);
        }

        self.receive_rate_history.push_back(self.receive_rate);
        self.transmit_rate_history.push_back(self.transmit_rate);

        if self.receive_rate_history.len() > HISTORY_SIZE {
            self.receive_rate_history.pop_front();
        }

        if self.transmit_rate_history.len() > HISTORY_SIZE {
            self.transmit_rate_history.pop_front();
        }

        self.last_update = now;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_received_bytes(&self) -> u64 {
        self.received_bytes
    }

    pub fn get_transmitted_bytes(&self) -> u64 {
        self.transmitted_bytes
    }

    pub fn get_received_packets(&self) -> u64 {
        self.received_packets
    }

    pub fn get_transmitted_packets(&self) -> u64 {
        self.transmitted_packets
    }

    pub fn get_receive_rate(&self) -> f64 {
        self.receive_rate
    }

    pub fn get_transmit_rate(&self) -> f64 {
        self.transmit_rate
    }

    pub fn get_receive_rate_history(&self) -> &VecDeque<f64> {
        &self.receive_rate_history
    }

    pub fn get_transmit_rate_history(&self) -> &VecDeque<f64> {
        &self.transmit_rate_history
    }
}

pub struct NetworkState {
    system: System,
    interfaces: HashMap<String, NetworkInterface>,
}

impl NetworkState {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_networks_list();

        let interfaces = system
            .networks()
            .into_iter()
            .map(|(name, data)| (name.clone(), NetworkInterface::new(name, data)))
            .collect();

        Self { system, interfaces }
    }

    pub fn update(&mut self) {
        self.system.refresh_networks();

        for (name, data) in self.system.networks() {
            if let Some(interface) = self.interfaces.get_mut(name) {
                interface.update(data);
            } else {
                self.interfaces
                    .insert(name.clone(), NetworkInterface::new(name, data));
            }
        }
    }

    pub fn get_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    pub fn get_total_received(&self) -> u64 {
        self.interfaces.values().map(|i| i.received_bytes).sum()
    }

    pub fn get_total_transmitted(&self) -> u64 {
        self.interfaces.values().map(|i| i.transmitted_bytes).sum()
    }

    pub fn get_total_receive_rate(&self) -> f64 {
        self.interfaces.values().map(|i| i.receive_rate).sum()
    }

    pub fn get_total_transmit_rate(&self) -> f64 {
        self.interfaces.values().map(|i| i.transmit_rate).sum()
    }
}
