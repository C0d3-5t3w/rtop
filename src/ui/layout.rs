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
) {
    // Create the main vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // CPU
            Constraint::Length(5),  // Memory
            Constraint::Length(8),  // Disk
            Constraint::Length(8),  // Network
            Constraint::Min(10),    // Processes (takes remaining space)
        ].as_ref())
        .split(frame.size());

    // Render each widget based on layout configuration
    if config.layout.show_cpu {
        widgets::render_cpu_widget(frame, main_chunks[0], &system.cpu, theme);
    }

    if config.layout.show_memory {
        widgets::render_memory_widget(frame, main_chunks[1], &system.memory, theme);
    }

    if config.layout.show_disk {
        widgets::render_disk_widget(frame, main_chunks[2], &system.disk, theme);
    }

    if config.layout.show_network {
        widgets::render_network_widget(frame, main_chunks[3], &system.network, theme);
    }

    // Always show processes, but details might be configurable
    widgets::render_process_widget(frame, main_chunks[4], &system.processes, config, theme);
}

// Alternative layouts that can be toggled

pub fn render_cpu_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70),  // CPU (large)
            Constraint::Percentage(30),  // Processes (smaller)
        ].as_ref())
        .split(frame.size());

    widgets::render_cpu_widget(frame, chunks[0], &system.cpu, theme);
    widgets::render_process_widget(frame, chunks[1], &system.processes, config, theme);
}

pub fn render_memory_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70),  // Memory (large)
            Constraint::Percentage(30),  // Processes (smaller)
        ].as_ref())
        .split(frame.size());

    widgets::render_memory_widget(frame, chunks[0], &system.memory, theme);
    widgets::render_process_widget(frame, chunks[1], &system.processes, config, theme);
}

pub fn render_compact<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
) {
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(frame.size());

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
}
