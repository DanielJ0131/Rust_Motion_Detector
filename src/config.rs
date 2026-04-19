use std::time::Duration;

/// Wi-Fi SSID, set via environment variable at compile time.
pub const WIFI_SSID: &str = env!("WIFI_SSID");

/// Wi-Fi password, set via environment variable at compile time.
pub const WIFI_PASS: &str = env!("WIFI_PASS");

/// Remote server URL for HTTP notifications, set via environment variable at compile time.
pub const PI_URL: &str = env!("PI_URL");

/// Cooldown duration (in seconds) between motion triggers.
pub const COOLDOWN: Duration = Duration::from_secs(10);
