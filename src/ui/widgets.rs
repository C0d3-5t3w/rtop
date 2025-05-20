use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{
        Block, Borders, Gauge, Row, Table,
    },
    Frame,
};

use crate::system::{
    CpuState, MemoryState, DiskState, NetworkState, ProcessList,
};
use crate::config::Config;
use crate::ui::theme::Theme;

// CPU Widget
pub fn render_cpu_widget<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    cpu: &CpuState,
    theme: &Theme,
) {
    let cpu_usage = cpu.get_average_usage();
    // Cap the CPU usage to 100.0 to prevent overflow
    let capped_usage = cpu_usage.min(100.0);
    
    // CPU Usage Gauge
    let gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(theme.cpu_color(capped_usage)))
        .percent(capped_usage as u16);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(1)].as_ref())
        .split(area);
    
    f.render_widget(gauge, chunks[0]);
}

// Memory Widget
pub fn render_memory_widget<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    memory: &MemoryState,
    theme: &Theme,
) {
    // Cap at 100% to prevent overflow
    let memory_usage = (memory.get_memory_usage_percent() as u16).min(100);
    let swap_usage = (memory.get_swap_usage_percent() as u16).min(100);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);
    
    // Memory Usage
    let memory_gauge = Gauge::default()
        .block(Block::default().title("Memory").borders(Borders::ALL))
        .gauge_style(Style::default().fg(theme.memory_color(memory_usage as f32)))
        .percent(memory_usage);
    
    // Swap Usage
    let swap_gauge = Gauge::default()
        .block(Block::default().title("Swap").borders(Borders::ALL))
        .gauge_style(Style::default().fg(theme.memory_color(swap_usage as f32)))
        .percent(swap_usage);
    
    f.render_widget(memory_gauge, chunks[0]);
    f.render_widget(swap_gauge, chunks[1]);
}

// Process List Widget
pub fn render_process_widget<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    processes: &ProcessList,
    config: &Config,
    theme: &Theme,
) {
    let block = Block::default().borders(Borders::ALL).title("Processes");
    
    let header_cells = ["PID", "CPU%", "MEM", "Name", "Status"].iter().map(|h| {
        Span::styled(*h, Style::default().fg(theme.header_color()))
    });
    let header = Row::new(header_cells).style(Style::default());
    
    // Get top processes by CPU or memory based on config
    let processes = if config.sort_by == "cpu" {
        processes.get_sorted_by_cpu(Some(area.height as usize - 3))
    } else {
        processes.get_sorted_by_memory(Some(area.height as usize - 3))
    };
    
    let rows = processes.iter().map(|p| {
        let pid = p.get_pid().to_string();
        let cpu = format!("{:.1}%", p.get_cpu_usage());
        let mem = format!("{} MB", p.get_memory_usage() / 1024);
        let name = p.get_name();
        let status = p.get_status();
        
        let row_data = vec![pid, cpu, mem, name.to_string(), status.to_string()];
        let cells = row_data.into_iter().map(|c| Span::raw(c));
        
        Row::new(cells)
    });
    
    let widths = [
        Constraint::Length(7),
        Constraint::Length(6),
        Constraint::Length(9),
        Constraint::Percentage(70),
        Constraint::Length(8),
    ];
    
    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    
    f.render_widget(table, area);
}

// Disk Usage Widget
pub fn render_disk_widget<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    disk: &DiskState,
    theme: &Theme,
) {
    let block = Block::default().borders(Borders::ALL).title("Disk Usage");
    
    let header_cells = ["Mount", "Size", "Used", "Avail", "Use%", "FS"].iter().map(|h| {
        Span::styled(*h, Style::default().fg(theme.header_color()))
    });
    let header = Row::new(header_cells).style(Style::default());
    
    let disks = disk.get_disks();
    let rows = disks.iter().map(|d| {
        let mount = d.get_mount_point().to_string();
        let size = format!("{:.1}G", d.get_total_space() as f64 / 1_073_741_824.0); // GB
        let used = format!("{:.1}G", d.get_used_space() as f64 / 1_073_741_824.0);
        let avail = format!("{:.1}G", d.get_available_space() as f64 / 1_073_741_824.0);
        let use_percent = format!("{:.1}%", d.get_usage_percent());
        let fs = d.get_file_system().to_string();
        
        let row_data = vec![mount, size, used, avail, use_percent, fs];
        let cells = row_data.into_iter().map(|c| Span::raw(c));
        
        Row::new(cells)
    });
    
    let widths = [
        Constraint::Percentage(20),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(10),
    ];
    
    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths);
    
    f.render_widget(table, area);
}

// Network Widget
pub fn render_network_widget<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    network: &NetworkState,
    theme: &Theme,
) {
    let block = Block::default().borders(Borders::ALL).title("Network");
    
    let header_cells = ["Interface", "RX", "TX", "RX/s", "TX/s"].iter().map(|h| {
        Span::styled(*h, Style::default().fg(theme.header_color()))
    });
    let header = Row::new(header_cells).style(Style::default());
    
    let interfaces = network.get_interfaces();
    let rows = interfaces.iter().map(|i| {
        let name = i.get_name().to_string();
        // Convert bytes to appropriate units
        let rx = format_bytes(i.get_received_bytes());
        let tx = format_bytes(i.get_transmitted_bytes());
        let rx_rate = format_bytes_rate(i.get_receive_rate());
        let tx_rate = format_bytes_rate(i.get_transmit_rate());
        
        let row_data = vec![name, rx, tx, rx_rate, tx_rate];
        let cells = row_data.into_iter().map(|c| Span::raw(c));
        
        Row::new(cells)
    });
    
    let widths = [
        Constraint::Percentage(20),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];
    
    let table = Table::new(rows)
        .header(header)
        .block(block)
        .widths(&widths);
    
    f.render_widget(table, area);
}

// Helper function to format bytes
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

// Helper function to format bytes per second
fn format_bytes_rate(bytes_per_sec: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    
    if bytes_per_sec >= GB {
        format!("{:.2} GB/s", bytes_per_sec / GB)
    } else if bytes_per_sec >= MB {
        format!("{:.2} MB/s", bytes_per_sec / MB)
    } else if bytes_per_sec >= KB {
        format!("{:.2} KB/s", bytes_per_sec / KB)
    } else {
        format!("{:.2} B/s", bytes_per_sec)
    }
}
