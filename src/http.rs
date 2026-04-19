use anyhow::Result;
use embedded_svc::http::client::Client;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use std::time::Duration;

use crate::config::PI_URL;

pub struct HttpClient {
    config: Configuration,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            config: Configuration {
                timeout: Some(Duration::from_secs(20)),
                ..Default::default()
            },
        }
    }

    pub fn send_trigger(&self) -> Result<u16> {
        let connection = EspHttpConnection::new(&self.config)?;
        let mut client = Client::wrap(connection);
        let response = client.get(PI_URL)?.submit()?;
        Ok(response.status())
    }
}