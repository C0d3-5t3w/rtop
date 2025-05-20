use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::config::Config;
use crate::system::SystemState;
use crate::ui::theme::Theme;
use crate::ui::widgets;

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
        .split(frame.size());

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Min(10),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

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

    widgets::render_process_widget(frame, content_chunks[4], &system.processes, config, theme);

    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_cpu_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
        .split(frame.size());

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(main_chunks[0]);

    widgets::render_cpu_widget(frame, content_chunks[0], &system.cpu, theme);
    widgets::render_process_widget(frame, content_chunks[1], &system.processes, config, theme);

    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_memory_focused<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
        .split(frame.size());

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(main_chunks[0]);

    widgets::render_memory_widget(frame, content_chunks[0], &system.memory, theme);
    widgets::render_process_widget(frame, content_chunks[1], &system.processes, config, theme);

    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_compact<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
        .split(frame.size());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(8),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(horizontal_chunks[1]);

    widgets::render_cpu_widget(frame, left_chunks[0], &system.cpu, theme);
    widgets::render_memory_widget(frame, left_chunks[1], &system.memory, theme);
    widgets::render_process_widget(frame, left_chunks[2], &system.processes, config, theme);

    widgets::render_disk_widget(frame, right_chunks[0], &system.disk, theme);
    widgets::render_network_widget(frame, right_chunks[1], &system.network, theme);

    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}

pub fn render_with_graphs<B: Backend>(
    frame: &mut Frame<B>,
    system: &SystemState,
    config: &Config,
    theme: &Theme,
    layout_name: &str,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)].as_ref())
        .split(frame.size());

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunks[0]);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(content_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(content_chunks[1]);

    widgets::render_cpu_graph(frame, top_chunks[0], &system.cpu, theme);
    widgets::render_memory_graph(frame, top_chunks[1], &system.memory, theme);
    widgets::render_network_graph(frame, bottom_chunks[0], &system.network, theme);
    widgets::render_process_widget(frame, bottom_chunks[1], &system.processes, config, theme);

    widgets::render_status_bar(frame, main_chunks[1], layout_name);
}
