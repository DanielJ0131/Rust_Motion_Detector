use anyhow::Result;
use embedded_svc::http::client::Client;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration as WifiConfig, EspWifi};
use std::time::{Duration, Instant};

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");
const PI_URL: &str = env!("PI_URL");
const COOLDOWN: Duration = Duration::from_secs(10);

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio48)?;
    
    // --- STEP 1: BOOTED ---
    blink(&mut led, 2, 70); 

    FreeRtos::delay_ms(4000);

    // --- STEP 2: INIT SERVICES ---
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let pir = PinDriver::input(peripherals.pins.gpio5, Pull::Down)?;
    blink(&mut led, 1, 300); 

    // --- STEP 3: INIT WIFI ---
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;
    wifi.set_configuration(&WifiConfig::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASS.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal, 
        ..Default::default()
    }))?;
    blink(&mut led, 2, 300); 

    // --- STEP 4: START WIFI ---
    wifi.start()?;
    FreeRtos::delay_ms(1000);
    blink(&mut led, 3, 300); 

    wifi.connect()?;
    
    let mut attempts = 0;
    while !wifi.is_connected()? {
        attempts += 1;
        led.set_high()?;
        FreeRtos::delay_ms(100);
        led.set_low()?;
        FreeRtos::delay_ms(900); 

        if attempts > 20 {
            let _ = wifi.disconnect();
            FreeRtos::delay_ms(1000);
            let _ = wifi.connect();
            attempts = 0;
        }
    }
    
    led.set_high()?;
    FreeRtos::delay_ms(2000);
    led.set_low()?;

    let mut last_pir_state = false;
    let mut last_trigger_time: Option<Instant> = None;
    let http_config = Configuration {
        timeout: Some(Duration::from_secs(20)), 
        ..Default::default()
    };

    loop {
        let current_pir_state = pir.is_high();
        if current_pir_state { let _ = led.set_high(); } else { let _ = led.set_low(); }

        if current_pir_state && !last_pir_state {
            let now = Instant::now();
            let can_trigger = match last_trigger_time {
                Some(last) => now.duration_since(last) >= COOLDOWN,
                None => true,
            };

            if can_trigger {
                match send_pi_request(&http_config) {
                    Ok(_) => {
                        led.set_high().ok();
                        FreeRtos::delay_ms(1500);
                        led.set_low().ok();
                    },
                    Err(_) => {
                        blink(&mut led, 3, 100);
                    },
                }
                last_trigger_time = Some(now);
            }
        }
        last_pir_state = current_pir_state;
        FreeRtos::delay_ms(100);
    }
}

// FIXED BLINK FUNCTION
fn blink(led: &mut PinDriver<'_, Output>, count: u32, ms: u32) {
    for _ in 0..count {
        let _ = led.set_high();
        FreeRtos::delay_ms(ms);
        let _ = led.set_low();
        FreeRtos::delay_ms(ms);
    }
    FreeRtos::delay_ms(500); 
}

fn send_pi_request(config: &Configuration) -> Result<u16> {
    let connection = EspHttpConnection::new(config)?;
    let mut client = Client::wrap(connection);
    let response = client.get(PI_URL)?.submit()?;
    Ok(response.status())
}