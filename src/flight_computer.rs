use crate::command::{Command, CommandProcessor};
use crate::errors::FlightComputerError;
use crate::telemetry::TelemetryHub;
use std::{sync::Arc, time::Duration};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
    sync::Mutex,
    time::interval,
};

/// Represents the FlightComputer, which unifies both commands and telemetry handling.
pub struct FlightComputer {
    /// The port used for recieving commands.
    pub command_port: u16,
    /// The port used for telemetry logs.
    pub log_port: u16,
    /// The telemetry hub that manages telemetry data.
    pub telemetry_hub: TelemetryHub,
}

impl FlightComputer {
    /// Creates a new instance of the `FlightComputer`.
    ///
    /// # Arguments
    ///
    /// * `command_port` - The port for incoming commands.
    /// * `log_port` - The port for telemetry logs.
    ///
    /// # Returns
    ///
    /// A new `FlightComputer` instance.
    pub fn new(command_port: u16, log_port: u16) -> Self {
        Self {
            command_port,
            log_port,
            telemetry_hub: TelemetryHub::new(),
        }
    }

    /// Runs the main functionality of the `FlightComputer`.
    ///
    /// This method spawns the telemetry service, starts the periodic tick loop,
    /// and listens for incoming commands.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `FlightComputerError`.
    pub async fn run(self) -> Result<(), FlightComputerError> {
        self.spawn_telemetry_server();
        let processor = Arc::new(Mutex::new(CommandProcessor::new(
            self.telemetry_hub.clone(),
        )));
        self.spawn_tick_loop(processor.clone());
        self.listen_for_commands(processor).await
    }

    /// Spawns the telemetry server to handle telemetry data.
    ///
    /// This server listens for incoming connections and adds them
    /// to the telemetry hub.
    fn spawn_telemetry_server(&self) {
        let telemetry_hub = self.telemetry_hub.clone();
        let port = self.log_port;
        tokio::spawn(async move {
            let listener = TcpListener::bind(("127.0.0.1", port))
                .await
                .expect("Failed to bind telemetry port");
            println!("Flight computer ready to telemeter data on port {}.", port);
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("New telemetry client connected: {}", addr);
                        telemetry_hub.add_client(stream).await;
                    }
                    Err(e) => eprintln!("Telemetry listener error: {}", e),
                }
            }
        });
    }

    /// Spawns the periodic tick loop for handling tasks.
    ///
    /// This loop runs at a fixed interval and processes periodic tasks
    /// using the `CommandProcessor`.
    ///
    /// This definitely wastes CPU cycles, but is the simplest method to handle scheduling.
    fn spawn_tick_loop(&self, processor: Arc<Mutex<CommandProcessor>>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_micros(100));
            loop {
                interval.tick().await;
                processor.lock().await.tick().await;
            }
        });
    }

    /// Listens for incoming commands from clients.
    ///
    /// This method accepts conections on the command port and processes
    /// incoming commands using the `CommandProcessor`.
    ///
    /// # Arguments
    ///
    /// * `processor` - A shared `CommandProcessor` instance for handling commands.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `FlightComputerError`.
    async fn listen_for_commands(
        &self,
        processor: Arc<Mutex<CommandProcessor>>,
    ) -> Result<(), FlightComputerError> {
        let listener = TcpListener::bind(("127.0.0.1", self.command_port)).await?;
        println!(
            "\nFlight computer ready for commands on port {}.",
            self.command_port
        );

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New command client connected: {}", addr);

            let processor = processor.clone();
            tokio::spawn(async move {
                let mut lines = BufReader::new(stream).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    match serde_json::from_str::<Command>(&line) {
                        Ok(cmd) => processor.lock().await.handle(cmd).await,
                        Err(e) => eprintln!("Invalid command: {} ({})", line, e),
                    }
                }
            });
        }
    }
}
