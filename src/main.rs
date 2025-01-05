mod app;
mod simulator;
mod config;
mod utils;
mod ui;
mod constants;
mod error;
mod logging;

use log::LevelFilter;
use crate::error::Result;

fn main() -> Result<()> {
    // Initialize logging with environment variable or default to Info
    let log_level = std::env::var("RUST_LOG")
        .map(|s| s.parse().unwrap_or(LevelFilter::Info))
        .unwrap_or(LevelFilter::Info);
    
    logging::init(log_level)?;
    log::info!("Starting Input Simulator");
    
    cosmic::app::run::<app::InputSimulatorApp>(ui::default_window_settings(), ())
        .map_err(|e| e.into())
}