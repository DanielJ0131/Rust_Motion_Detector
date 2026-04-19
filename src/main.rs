mod config;
mod http;
mod led;
mod wifi;

use anyhow::Result;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use std::time::Instant;

use crate::config::COOLDOWN;
use crate::http::HttpClient;
use crate::led::blink;

/// Entry point for the Rust Motion Detector application.
///
/// This function initializes hardware peripherals, configures Wi-Fi, sets up the PIR sensor and LED,
/// and enters the main event loop for motion detection. When motion is detected, it sends an HTTP GET
/// request to a remote server and provides visual feedback via the onboard LED. Cooldown logic prevents
/// repeated triggers within a short window.
///
/// # Steps
/// 1. Boot indication via LED.
/// 2. Initialize system services and peripherals.
/// 3. Connect to Wi-Fi and indicate status via LED.
/// 4. Main loop: poll PIR sensor, trigger HTTP request on motion, and handle LED feedback.
///
/// # Errors
/// Returns an error if hardware initialization or Wi-Fi connection fails.
fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio48)?;

    // --- STEP 1: BOOT ---
    blink(&mut led, 2, 70);
    FreeRtos::delay_ms(4000);

    // --- STEP 2: INIT SERVICES ---
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let pir = PinDriver::input(peripherals.pins.gpio5, Pull::Down)?;

    blink(&mut led, 1, 300);

    // --- STEP 3 & 4: START WIFI ---
    let _wifi = wifi::init_and_connect(peripherals.modem, sys_loop, nvs, &mut led)?;

    let http_client = HttpClient::new();
    let mut last_pir_state = false;
    let mut last_trigger_time: Option<Instant> = None;

    // --- MAIN LOOP ---
    loop {
        let current_pir_state = pir.is_high();
        if current_pir_state {
            let _ = led.set_high();
        } else {
            let _ = led.set_low();
        }

        if current_pir_state && !last_pir_state {
            let now = Instant::now();
            let can_trigger = match last_trigger_time {
                Some(last) => now.duration_since(last) >= COOLDOWN,
                None => true,
            };

            if can_trigger {
                match http_client.send_trigger() {
                    Ok(_) => {
                        let _ = led.set_high();
                        FreeRtos::delay_ms(1500);
                        let _ = led.set_low();
                    }
                    Err(_) => {
                        blink(&mut led, 3, 100);
                    }
                }
                last_trigger_time = Some(now);
            }
        }

        last_pir_state = current_pir_state;
        FreeRtos::delay_ms(100);
    }
}
