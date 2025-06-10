use std::io;
use thiserror::Error;

/// Defines errors for the `FlightComputer`.
///
/// The `thiserror` crate ensures type-safe and concise error handling.
#[derive(Debug, Error)]
pub enum FlightComputerError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid command received: {0}")]
    InvalidCommand(#[from] serde_json::Error),
}
