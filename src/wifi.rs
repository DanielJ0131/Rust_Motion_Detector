use anyhow::Result;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Output, PinDriver},
    modem::Modem,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, ClientConfiguration, Configuration as WifiConfig, EspWifi},
};

use crate::config::{WIFI_PASS, WIFI_SSID};
use crate::led::blink;

/// Initializes and connects the ESP32-S3 to a Wi-Fi network.
///
/// This function sets up the Wi-Fi driver, configures the client with credentials,
/// provides visual feedback via the onboard LED, and handles connection retries.
///
/// # Arguments
/// * `modem`    - ESP32 modem peripheral.
/// * `sys_loop` - System event loop for Wi-Fi events.
/// * `nvs`      - Non-volatile storage partition for Wi-Fi credentials.
/// * `led`      - Mutable reference to the status LED pin driver.
///
/// # Returns
/// * `Result<EspWifi<'static>>` - The initialized and connected Wi-Fi driver, or an error.
pub fn init_and_connect(
    modem: Modem<'static>,
    sys_loop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
    led: &mut PinDriver<'_, Output>,
) -> Result<EspWifi<'static>> {
    let mut wifi = EspWifi::new(modem, sys_loop, Some(nvs))?;

    wifi.set_configuration(&WifiConfig::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASS.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    blink(led, 2, 300);

    wifi.start()?;
    FreeRtos::delay_ms(1000);
    blink(led, 3, 300);

    wifi.connect()?;

    let mut attempts = 0;
    while !wifi.is_connected()? {
        attempts += 1;
        let _ = led.set_high();
        FreeRtos::delay_ms(100);
        let _ = led.set_low();
        FreeRtos::delay_ms(900);

        if attempts > 20 {
            let _ = wifi.disconnect();
            FreeRtos::delay_ms(1000);
            let _ = wifi.connect();
            attempts = 0;
        }
    }

    let _ = led.set_high();
    FreeRtos::delay_ms(2000);
    let _ = led.set_low();

    Ok(wifi)
}
