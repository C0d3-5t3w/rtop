mod cli;
mod config;
mod system;
mod ui;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let config = config::Config::load(args.config.as_deref())?;

    let mut app = ui::App::new(config);
    app.run()?;

    Ok(())
}
