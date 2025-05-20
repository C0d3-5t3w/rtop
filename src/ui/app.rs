use crate::config::Config;
use crate::system::SystemState;
use crate::ui::theme::Theme;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

pub struct App {
    system: SystemState,
    config: Config,
    theme: Theme,
    should_quit: bool,
    current_layout: LayoutView,
}

impl App {
    pub fn new(config: Config) -> Self {
        let theme = Theme::from_name(&config.theme);
        Self {
            system: SystemState::new(),
            config,
            theme,
            should_quit: false,
            current_layout: LayoutView::Default,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(self.config.update_interval);
        let res = self.run_app(&mut terminal, tick_rate);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    fn run_app<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> Result<()> {
        let mut last_tick = Instant::now();

        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key.code);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.update();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.system.update();
        })) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Error updating system metrics");
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') => self.theme.cycle_next(),
            KeyCode::Char('g') => self.toggle_graph_view(),
            KeyCode::Char('1') => self.current_layout = LayoutView::Default,
            KeyCode::Char('2') => self.current_layout = LayoutView::GraphView,
            KeyCode::Char('3') => self.current_layout = LayoutView::CpuFocused,
            KeyCode::Char('4') => self.current_layout = LayoutView::MemoryFocused,
            KeyCode::Char('5') => self.current_layout = LayoutView::Compact,

            _ => {}
        }
    }

    fn toggle_graph_view(&mut self) {
        self.current_layout = match self.current_layout {
            LayoutView::Default => LayoutView::GraphView,
            LayoutView::GraphView => LayoutView::Default,
            _ => LayoutView::Default,
        };
    }

    fn render<B: Backend>(&self, frame: &mut ratatui::Frame<B>) {
        match self.current_layout {
            LayoutView::Default => crate::ui::layout::render(
                frame,
                &self.system,
                &self.config,
                &self.theme,
                "Default View",
            ),
            LayoutView::GraphView => crate::ui::layout::render_with_graphs(
                frame,
                &self.system,
                &self.config,
                &self.theme,
                "Graph View",
            ),
            LayoutView::CpuFocused => crate::ui::layout::render_cpu_focused(
                frame,
                &self.system,
                &self.config,
                &self.theme,
                "CPU Focus",
            ),
            LayoutView::MemoryFocused => crate::ui::layout::render_memory_focused(
                frame,
                &self.system,
                &self.config,
                &self.theme,
                "Memory Focus",
            ),
            LayoutView::Compact => crate::ui::layout::render_compact(
                frame,
                &self.system,
                &self.config,
                &self.theme,
                "Compact View",
            ),
        }
    }
}

#[derive(Clone, Copy)]
enum LayoutView {
    Default,
    GraphView,
    CpuFocused,
    MemoryFocused,
    Compact,
}
