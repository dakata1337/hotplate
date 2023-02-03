use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let button1 = PinDriver::input(pins.gpio14).unwrap();

    loop {
        if button1.is_high() {
            println!("Hello, world!");
        }
    }
}
