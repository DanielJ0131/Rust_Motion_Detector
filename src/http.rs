use anyhow::Result;
use embedded_svc::http::client::Client;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use std::time::Duration;

use crate::config::PI_URL;

/// HTTP client for sending motion trigger notifications.
///
/// Wraps ESP-IDF HTTP client functionality and provides a simple
/// interface for sending GET requests to the configured PI_URL.
pub struct HttpClient {
    /// HTTP client configuration (e.g., timeout).
    config: Configuration,
}

impl HttpClient {
    /// Creates a new HttpClient with a 20-second timeout.
    pub fn new() -> Self {
        Self {
            config: Configuration {
                timeout: Some(Duration::from_secs(20)),
                ..Default::default()
            },
        }
    }

    /// Sends a GET request to the configured PI_URL to trigger an event.
    ///
    /// Returns the HTTP status code on success.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or the connection cannot be established.
    pub fn send_trigger(&self) -> Result<u16> {
        let connection = EspHttpConnection::new(&self.config)?;
        let mut client = Client::wrap(connection);
        let response = client.get(PI_URL)?.submit()?;
        Ok(response.status())
    }
}
