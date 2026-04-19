use std::time::Duration;

pub const WIFI_SSID: &str = env!("WIFI_SSID");
pub const WIFI_PASS: &str = env!("WIFI_PASS");
pub const PI_URL: &str = env!("PI_URL");
pub const COOLDOWN: Duration = Duration::from_secs(10);
