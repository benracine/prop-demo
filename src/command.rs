use crate::telemetry::TelemetryHub;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::Instant;

/// Represents a command with a delay in seconds.
#[derive(Debug, Deserialize, Serialize)]
pub struct Command(pub f64);

/// Processes commands and manages scheduled propulsion events.
#[derive(Clone)]
pub struct CommandProcessor {
    /// The time at which the propulsion is scheduled to fire.
    scheduled_fire: Option<Instant>,
    /// The telemetry hub used to send telemetry data.
    telemetry: TelemetryHub,
}

impl CommandProcessor {
    /// Creates a new `CommandProcessor`.
    ///
    /// # Arguments
    ///
    /// * `telemetry` - The telemetry hub for sending telemetry data.
    pub fn new(telemetry: TelemetryHub) -> Self {
        Self {
            scheduled_fire: None,
            telemetry,
        }
    }

    /// Handles an incoming command.
    ///
    /// Depending on the delay value, it either cancels, shedules, or marks the command as invalid.
    ///
    /// # Arguments
    ///
    /// * `Command(delay)` - The command containing the delay value.
    pub async fn handle(&mut self, Command(delay): Command) {
        if delay == -1.0 {
            self.cancel().await;
        } else if delay >= 0.0 {
            self.schedule(delay).await;
        } else {
            self.invalid(delay).await;
        }
    }

    /// Cancels any scheduled propulsion event.
    async fn cancel(&mut self) {
        self.scheduled_fire = None;
        self.telemetry
            .send_telemetry("🛑", "Cancelled fire command")
            .await;
    }

    /// Schedules a propulsion event after the specified delay.
    ///
    /// # Arguments
    ///
    /// * `secs` - The delay in seconds before firing.
    async fn schedule(&mut self, secs: f64) {
        let when = Instant::now() + Duration::from_secs_f64(secs);
        self.scheduled_fire = Some(when);
        let msg = format!("Scheduled fire in {:.2}s", secs);
        self.telemetry.send_telemetry("🛰️ ⏳", &msg).await;
    }

    /// Marks a command as invalid and sends a telemetry message.
    ///
    /// # Arguments
    ///
    /// * `value` - The invalid delay value.
    async fn invalid(&self, value: f64) {
        let msg = format!("Invalid delay value: {}", value);
        self.telemetry.send_telemetry("⚠️", &msg).await;
    }

    /// Checks if it's time to fire propulsion and sends a telemetry message if so.
    pub async fn tick(&mut self) {
        if let Some(when) = self.scheduled_fire {
            if Instant::now() >= when {
                self.scheduled_fire = None;
                self.telemetry
                    .send_telemetry("🚀", "Firing propulsion now!")
                    .await;
            }
        }
    }
}
