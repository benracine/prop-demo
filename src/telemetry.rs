use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

/// A hub for managing telemetry data and sending it to a connected client.
#[derive(Clone)]
pub struct TelemetryHub {
    /// The telemetry client, wrapped in an `Arc<Mutex>` for thread-safe access.
    client: Arc<Mutex<Option<TcpStream>>>,
}

impl TelemetryHub {
    /// Creates a new `TelemetryHub` instance.
    ///
    /// # Returns
    ///
    /// A new `TelemetryHub` with no connected client.
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// Adds a telemetry client to the hub.
    ///
    /// # Arguments
    ///
    /// * `stream` - A `TcpStream` representing the client connection.
    pub async fn add_client(&self, stream: TcpStream) {
        *self.client.lock().await = Some(stream);
    }

    /// Sends a telemetry message to the connected client.
    ///
    /// # Arguments
    ///
    /// * `tag` - A short tag describing the telemetry message.
    /// * `payload` - The content of the telemetry message.
    ///
    /// If no client is connected, an error message is printed to the console.
    pub async fn send_telemetry(&self, tag: &str, payload: &str) {
        let msg = format!("[{}] {}\n", tag, payload);
        let mut guard = self.client.lock().await;

        match guard.as_mut() {
            Some(stream) => match stream.write_all(msg.as_bytes()).await {
                Ok(_) => {
                    // Telemetry sent successfully
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to send telemetry: {}", e);
                }
            },
            None => {
                eprintln!("⚠️ No telemetry client connected");
            }
        }
    }
}
