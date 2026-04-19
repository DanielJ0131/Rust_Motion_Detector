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