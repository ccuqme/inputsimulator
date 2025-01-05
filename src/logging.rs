use log::{Level, LevelFilter, Log, Metadata, Record};
use crate::error::{Result, AppError};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // Filter out spammy device maintenance messages
        if metadata.target().contains("wgpu") && 
           metadata.level() <= Level::Info {
            return false;
        }
        
        // For all other messages, use normal filtering
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<()> {
    let logger = Box::new(SimpleLogger);
    log::set_max_level(level);
    log::set_logger(Box::leak(logger))
        .map_err(|_| AppError::Logger)
}
