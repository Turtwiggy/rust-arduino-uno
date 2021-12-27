// 
// Blink the builtin LED - the "Hello World" of embedded programming.
// 

// link the core-crate instead of std-crate
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate arduino_hal;
// use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}