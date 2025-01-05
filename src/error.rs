use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Input simulation error: {0}")]
    Simulation(String),

    #[error("Device error: {0}")]
    Device(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cosmic error: {0}")]
    Cosmic(#[from] cosmic::iced::Error),

    #[error("Logger initialization error")]
    Logger,
}

pub type Result<T> = std::result::Result<T, AppError>;

// Simulator-specific errors
#[derive(Error, Debug)]
pub enum SimulatorError {
    #[error("Failed to simulate key: {0}")]
    KeySimulation(String),

    #[error("Failed to initialize device: {0}")]
    DeviceInitialization(String),
}

impl From<SimulatorError> for AppError {
    fn from(err: SimulatorError) -> Self {
        match err {
            SimulatorError::KeySimulation(msg) => AppError::Simulation(msg),
            SimulatorError::DeviceInitialization(msg) => AppError::Device(msg),
        }
    }
}