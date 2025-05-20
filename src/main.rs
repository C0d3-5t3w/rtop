mod cli;
mod system;
mod ui;
mod config;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    // Load configuration
    let config = config::Config::load(args.config.as_deref())?;
    
    // Create and run the app
    let mut app = ui::App::new(config);
    app.run()?;
    
    Ok(())
}
