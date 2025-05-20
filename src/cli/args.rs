use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ViewMode {
    Basic,
    Detailed,
    ProcessFocus,
    SystemFocus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ColorTheme {
    Default,
    Dark,
    Light,
    Custom,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Update interval in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,

    /// Display mode
    #[arg(short, long, value_enum, default_value_t = ViewMode::Basic)]
    pub view: ViewMode,

    /// Color theme
    #[arg(short, long, value_enum, default_value_t = ColorTheme::Default)]
    pub theme: ColorTheme,

    /// Path to configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Process filter string
    #[arg(short, long)]
    pub filter: Option<String>,
}
