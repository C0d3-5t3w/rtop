use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Color},
    text::{Span, Line},
    widgets::{
        Block, Borders, Gauge, Row, Table,
        Chart, Dataset, GraphType, Axis, Paragraph,
    },
    symbols,
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

// CPU Graph Widget
pub fn render_cpu_graph<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    cpu: &CpuState,
    theme: &Theme,
) {
    let block = Block::default()
        .title("CPU Usage History")
        .borders(Borders::ALL);
    
    let cpu_history = cpu.get_history();
    if cpu_history.is_empty() {
        f.render_widget(block, area);
        return;
    }
    
    // Create data points from history
    let data: Vec<(f64, f64)> = cpu_history
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value as f64))
        .collect();
    
    let datasets = vec![
        Dataset::default()
            .name("CPU %")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme.cpu_color(cpu.get_average_usage())))
            .data(&data),
    ];
    
    let chart = Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .title(Span::styled("Time", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, cpu_history.len() as f64])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("CPU %", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::styled("0", Style::default().fg(Color::Gray)),
                    Span::styled("50", Style::default().fg(Color::Gray)),
                    Span::styled("100", Style::default().fg(Color::Gray)),
                ]),
        );
    
    f.render_widget(chart, area);
}

// Memory Graph Widget
pub fn render_memory_graph<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    memory: &MemoryState,
    theme: &Theme,
) {
    let block = Block::default()
        .title("Memory & Swap History")
        .borders(Borders::ALL);
    
    let mem_history = memory.get_memory_history();
    let swap_history = memory.get_swap_history();
    
    if mem_history.is_empty() && swap_history.is_empty() {
        f.render_widget(block, area);
        return;
    }
    
    // Determine the length to use (should be the same for both)
    let length = std::cmp::max(mem_history.len(), swap_history.len());
    
    // Create data points from history
    let mem_data: Vec<(f64, f64)> = mem_history
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value))
        .collect();
    
    let swap_data: Vec<(f64, f64)> = swap_history
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value))
        .collect();
    
    let datasets = vec![
        Dataset::default()
            .name("Memory %")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme.memory_color(memory.get_memory_usage_percent() as f32)))
            .data(&mem_data),
        Dataset::default()
            .name("Swap %")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::LightMagenta))
            .data(&swap_data),
    ];
    
    let chart = Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .title(Span::styled("Time", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, length as f64])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("Usage %", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::styled("0", Style::default().fg(Color::Gray)),
                    Span::styled("50", Style::default().fg(Color::Gray)),
                    Span::styled("100", Style::default().fg(Color::Gray)),
                ]),
        );
    
    f.render_widget(chart, area);
}

// Network Graph Widget
pub fn render_network_graph<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    network: &NetworkState,
    _theme: &Theme,  // Renamed theme to _theme to show it's intentionally unused
) {
    let block = Block::default()
        .title("Network Traffic")
        .borders(Borders::ALL);
    
    // Get the primary network interface or return if none
    let interfaces = network.get_interfaces();
    if interfaces.is_empty() {
        f.render_widget(block, area);
        return;
    }
    
    // For simplicity, just show the first interface
    let interface = interfaces[0];
    let rx_history = interface.get_receive_rate_history();
    let tx_history = interface.get_transmit_rate_history();
    
    if rx_history.is_empty() && tx_history.is_empty() {
        f.render_widget(block, area);
        return;
    }
    
    // Determine the length to use (should be the same for both)
    let length = std::cmp::max(rx_history.len(), tx_history.len());
    
    // Scale the data to make the chart more readable
    // Find maximum value to auto-scale with explicit f64 type
    let max_rx = rx_history.iter().fold(0.0_f64, |a, &b| f64::max(a, b));
    let max_tx = tx_history.iter().fold(0.0_f64, |a, &b| f64::max(a, b));
    let max_value = f64::max(max_rx, max_tx);
    
    // Create scaled data points from history
    let rx_data: Vec<(f64, f64)> = rx_history
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let scaled = if max_value > 0.0 { (value / max_value) * 100.0 } else { 0.0 };
            (i as f64, scaled)
        })
        .collect();
    
    let tx_data: Vec<(f64, f64)> = tx_history
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let scaled = if max_value > 0.0 { (value / max_value) * 100.0 } else { 0.0 };
            (i as f64, scaled)
        })
        .collect();
    
    let datasets = vec![
        Dataset::default()
            .name(format!("RX: {}/s", format_bytes_rate(interface.get_receive_rate())))
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Blue))
            .data(&rx_data),
        Dataset::default()
            .name(format!("TX: {}/s", format_bytes_rate(interface.get_transmit_rate())))
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Red))
            .data(&tx_data),
    ];
    
    let max_label = format_bytes_rate(max_value);
    let half_label = format_bytes_rate(max_value / 2.0);
    
    let chart = Chart::new(datasets)
        .block(block)
        .x_axis(
            Axis::default()
                .title(Span::styled("Time", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, length as f64])
                .labels(vec![]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled("Throughput", Style::default().fg(Color::Gray)))
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::styled("0", Style::default().fg(Color::Gray)),
                    Span::styled(half_label, Style::default().fg(Color::Gray)),
                    Span::styled(max_label, Style::default().fg(Color::Gray)),
                ]),
        );
    
    f.render_widget(chart, area);
}

// Status Bar Widget for displaying controls
pub fn render_status_bar<B: ratatui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    current_layout: &str,
) {
    let controls = vec![
        ("q", "Quit"),
        ("c", "Cycle Theme"),
        ("g", "Graph View"),
        ("1-5", "Change Layout"),
        ("", current_layout),
    ];

    let mut control_spans = Vec::new();
    for (key, desc) in controls {
        if !key.is_empty() {
            control_spans.push(Span::styled(
                format!("[{}]", key),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            ));
            control_spans.push(Span::raw(" "));
            control_spans.push(Span::styled(
                format!("{}", desc),
                Style::default().fg(Color::White)
            ));
            control_spans.push(Span::raw("  "));
        } else if !desc.is_empty() {
            // For the layout name
            control_spans.push(Span::styled(
                format!("Current: {}", desc),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ));
        }
    }
    
    let status_line = Line::from(control_spans);
    
    let status_bar = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray));
    
    let paragraph = Paragraph::new(status_line)
        .block(status_bar)
        .alignment(ratatui::layout::Alignment::Center);
    
    f.render_widget(paragraph, area);
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
