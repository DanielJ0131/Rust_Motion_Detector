use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Output, PinDriver},
};

pub fn blink(led: &mut PinDriver<'_, Output>, count: u32, ms: u32) {
    for _ in 0..count {
        let _ = led.set_high();
        FreeRtos::delay_ms(ms);
        let _ = led.set_low();
        FreeRtos::delay_ms(ms);
    }
    FreeRtos::delay_ms(500);
}