use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::system::SystemState;
use crate::config::Config;
use crate::ui::theme::Theme;
use crate::ui::widgets;

// Main render function that coordinates layout and widgets
pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    // Split the screen into main content and status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // Main content
            Constraint::Length(2),    // Status bar
        ].as_ref())
        .split(frame.size());
    
    // Create the main vertical layout within the content area
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // CPU
            Constraint::Length(5),  // Memory
            Constraint::Length(8),  // Disk
            Constraint::Length(8),  // Network
            Constraint::Min(10),    // Processes (takes remaining space)
        ].as_ref())
        .split(main_chunks[0]);

    // Render each widget based on layout configuration
    if config.layout.show_cpu {
        widgets::render_cpu_widget(frame, content_chunks[0], &system.cpu, theme);
    }

    if config.layout.show_memory {
        widgets::render_memory_widget(frame, content_chunks[1], &system.memory, theme);
    }

    if config.layout.show_disk {
        widgets::render_disk_widget(frame, content_chunks[2], &system.disk, theme);
    }

    if config.layout.show_network {
        widgets::render_network_widget(frame, content_chunks[3], &system.network, theme);
    }

    // Always show processes, but details might be configurable
    widgets::render_process_widget(frame, content_chunks[4], &system.processes, config, theme);
    
    // Render the status bar at the bottom
    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

// Alternative layouts that can be toggled

pub fn render_cpu_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    // Split for main content and status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // Main content
            Constraint::Length(2),    // Status bar
        ].as_ref())
        .split(frame.size());
    
    // Content layout
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70),  // CPU (large)
            Constraint::Percentage(30),  // Processes (smaller)
        ].as_ref())
        .split(main_chunks[0]);

    widgets::render_cpu_widget(frame, content_chunks[0], &system.cpu, theme);
    widgets::render_process_widget(frame, content_chunks[1], &system.processes, config, theme);
    
    // Status bar
    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_memory_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    // Split for main content and status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // Main content
            Constraint::Length(2),    // Status bar
        ].as_ref())
        .split(frame.size());
    
    // Content layout
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70),  // Memory (large)
            Constraint::Percentage(30),  // Processes (smaller)
        ].as_ref())
        .split(main_chunks[0]);

    widgets::render_memory_widget(frame, content_chunks[0], &system.memory, theme);
    widgets::render_process_widget(frame, content_chunks[1], &system.processes, config, theme);
    
    // Status bar
    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_compact<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    // Split for main content and status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // Main content
            Constraint::Length(2),    // Status bar
        ].as_ref())
        .split(frame.size());
    
    // Content layout
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(main_chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // CPU
            Constraint::Length(5),  // Memory
            Constraint::Min(8),     // Remaining space
        ].as_ref())
        .split(horizontal_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),  // Disk
            Constraint::Percentage(50),  // Network
        ].as_ref())
        .split(horizontal_chunks[1]);

    widgets::render_cpu_widget(frame, left_chunks[0], &system.cpu, theme);
    widgets::render_memory_widget(frame, left_chunks[1], &system.memory, theme);
    widgets::render_process_widget(frame, left_chunks[2], &system.processes, config, theme);
    
    widgets::render_disk_widget(frame, right_chunks[0], &system.disk, theme);
    widgets::render_network_widget(frame, right_chunks[1], &system.network, theme);
    
    // Status bar
    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_with_graphs<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    // Split for main content and status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),       // Main content
            Constraint::Length(2),    // Status bar
        ].as_ref())
        .split(frame.size());
    
    // Content layout    
    // Create a 2x2 grid layout for graphs and data
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),  // Top half
            Constraint::Percentage(50),  // Bottom half
        ].as_ref())
        .split(main_chunks[0]);
    
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // CPU
            Constraint::Percentage(50),  // Memory
        ].as_ref())
        .split(content_chunks[0]);
    
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // Network
            Constraint::Percentage(50),  // Processes
        ].as_ref())
        .split(content_chunks[1]);
    
    // Render the graphs and tables
    widgets::render_cpu_graph(frame, top_chunks[0], &system.cpu, theme);
    widgets::render_memory_graph(frame, top_chunks[1], &system.memory, theme);
    widgets::render_network_graph(frame, bottom_chunks[0], &system.network, theme);
    widgets::render_process_widget(frame, bottom_chunks[1], &system.processes, config, theme);
    
    // Status bar
    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}
