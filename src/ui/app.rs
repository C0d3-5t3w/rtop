use crate::system::SystemState;
use crate::config::Config;
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
use std::{io, time::{Duration, Instant}};

pub struct App {
    system: SystemState,
    config: Config,
    theme: Theme,
    should_quit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let theme = Theme::from_name(&config.theme);
        Self {
            system: SystemState::new(),
            config,
            theme,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(self.config.update_interval);
        let res = self.run_app(&mut terminal, tick_rate);

        // Restore terminal
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

    fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>, tick_rate: Duration) -> Result<()> {
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
        self.system.update();
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') => self.cycle_theme(),
            // Add more key handlers for customization
            _ => {}
        }
    }

    fn cycle_theme(&mut self) {
        self.theme.cycle_next();
    }

    fn render<B: Backend>(&self, frame: &mut ratatui::Frame<B>) {
        crate::ui::layout::render(frame, &self.system, &self.config, &self.theme);
    }
}
