use prop_command_demo::{
    errors::FlightComputerError, flight_computer::FlightComputer, telemetry::TelemetryHub,
};
use tokio::signal;

#[tokio::main]
/// The main function initializes the `FlightComputer` and starts its main loop.
///
/// # Returns
///
/// A `Result` indicating success or a `FlightComputerError`.
async fn main() -> Result<(), FlightComputerError> {
    // Initialize the FlightComputer with ports and telemetry hub
    let fc = FlightComputer {
        command_port: 8124,
        log_port: 8125,
        telemetry_hub: TelemetryHub::new().clone(),
    };

    // Spawn the FlightComputer's main run loop as an asynchronous task
    let fc_handle = tokio::spawn(fc.run());

    // Wait for a Ctrl+C signal to gracefully shut down the application
    signal::ctrl_c().await.map_err(FlightComputerError::Io)?;

    // Wait for the FlightComputer task to finish and handle any errors
    match fc_handle.await {
        Ok(Ok(())) => Ok(()), // The task completed successfully
        Ok(Err(e)) => Err(e), // The task returned an error
        Err(e) => {
            eprintln!("FlightComputer task panicked: {:?}", e);
            Err(FlightComputerError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "FlightComputer task panicked",
            )))
        }
    }
}
