use ratatui::style::Color;
use std::collections::HashMap;

pub struct Theme {
    name: String,
    colors: HashMap<String, Color>,
}

impl Theme {
    pub fn new(name: &str) -> Self {
        let mut theme = Self {
            name: name.to_string(),
            colors: HashMap::new(),
        };
        theme.set_defaults();
        theme
    }

    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dark" => Self::dark(),
            "light" => Self::light(),
            "custom" => Self::custom(),
            _ => Self::default_theme(),
        }
    }

    fn set_defaults(&mut self) {
        self.colors.insert("background".to_string(), Color::Black);
        self.colors.insert("foreground".to_string(), Color::White);
        self.colors.insert("header".to_string(), Color::Cyan);
        self.colors.insert("cpu_low".to_string(), Color::Green);
        self.colors.insert("cpu_medium".to_string(), Color::Yellow);
        self.colors.insert("cpu_high".to_string(), Color::Red);
        self.colors.insert("memory_low".to_string(), Color::Green);
        self.colors.insert("memory_medium".to_string(), Color::Yellow);
        self.colors.insert("memory_high".to_string(), Color::Red);
        self.colors.insert("disk_low".to_string(), Color::Green);
        self.colors.insert("disk_medium".to_string(), Color::Yellow);
        self.colors.insert("disk_high".to_string(), Color::Red);
        self.colors.insert("network_rx".to_string(), Color::Blue);
        self.colors.insert("network_tx".to_string(), Color::Magenta);
        self.colors.insert("process_selected".to_string(), Color::Cyan);
        self.colors.insert("border".to_string(), Color::Gray);
        self.colors.insert("tab_active".to_string(), Color::Cyan);
        self.colors.insert("tab_inactive".to_string(), Color::Gray);
    }

    pub fn default_theme() -> Self {
        Self::new("default")
    }

    pub fn dark() -> Self {
        let mut theme = Self::new("dark");
        theme.colors.insert("background".to_string(), Color::Black);
        theme.colors.insert("foreground".to_string(), Color::Gray);
        theme.colors.insert("header".to_string(), Color::Cyan);
        theme.colors.insert("border".to_string(), Color::DarkGray);
        theme
    }

    pub fn light() -> Self {
        let mut theme = Self::new("light");
        theme.colors.insert("background".to_string(), Color::White);
        theme.colors.insert("foreground".to_string(), Color::Black);
        theme.colors.insert("header".to_string(), Color::Blue);
        theme.colors.insert("border".to_string(), Color::Gray);
        theme.colors.insert("cpu_low".to_string(), Color::Green);
        theme.colors.insert("cpu_medium".to_string(), Color::Yellow);
        theme.colors.insert("cpu_high".to_string(), Color::Red);
        theme
    }

    pub fn custom() -> Self {
        // This would ideally load from a config file
        Self::default_theme()
    }

    pub fn get_color(&self, name: &str) -> Color {
        self.colors.get(name).copied().unwrap_or(Color::White)
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn cycle_next(&mut self) {
        self.name = match self.name.as_str() {
            "default" => "dark".to_string(),
            "dark" => "light".to_string(),
            "light" => "custom".to_string(),
            _ => "default".to_string(),
        };
        
        *self = Self::from_name(&self.name);
    }

    // Specific color utilities
    pub fn cpu_color(&self, usage: f32) -> Color {
        if usage < 50.0 {
            self.get_color("cpu_low")
        } else if usage < 80.0 {
            self.get_color("cpu_medium")
        } else {
            self.get_color("cpu_high")
        }
    }

    pub fn memory_color(&self, usage: f32) -> Color {
        if usage < 50.0 {
            self.get_color("memory_low")
        } else if usage < 80.0 {
            self.get_color("memory_medium")
        } else {
            self.get_color("memory_high")
        }
    }

    #[allow(dead_code)]
    pub fn disk_color(&self, usage: f32) -> Color {
        if usage < 70.0 {
            self.get_color("disk_low")
        } else if usage < 90.0 {
            self.get_color("disk_medium")
        } else {
            self.get_color("disk_high")
        }
    }

    pub fn header_color(&self) -> Color {
        self.get_color("header")
    }

    #[allow(dead_code)]
    pub fn border_color(&self) -> Color {
        self.get_color("border")
    }

    #[allow(dead_code)]
    pub fn background_color(&self) -> Color {
        self.get_color("background")
    }

    #[allow(dead_code)]
    pub fn foreground_color(&self) -> Color {
        self.get_color("foreground")
    }
}
