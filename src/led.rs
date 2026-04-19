use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Output, PinDriver},
};

/// Blinks an LED a specified number of times with a given delay.
///
/// # Arguments
/// * `led`   - Mutable reference to a PinDriver for the output pin (LED).
/// * `count` - Number of blink cycles to perform.
/// * `ms`    - Duration in milliseconds for each on/off state.
///
/// After blinking, waits an additional 500ms before returning.
pub fn blink(led: &mut PinDriver<'_, Output>, count: u32, ms: u32) {
    for _ in 0..count {
        let _ = led.set_high();
        FreeRtos::delay_ms(ms);
        let _ = led.set_low();
        FreeRtos::delay_ms(ms);
    }
    FreeRtos::delay_ms(500);
}
