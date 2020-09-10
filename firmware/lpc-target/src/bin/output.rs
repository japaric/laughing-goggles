#![no_main]
#![no_std]

use lpc8xx_hal::{gpio, Peripherals};
use lpc_target as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syscon = p.SYSCON.split();
    let gpio = p.GPIO.enable(&mut syscon.handle);
    let _pin31 = p
        .pins
        .pio1_0
        .into_output_pin(gpio.tokens.pio1_0, gpio::Level::Low);

    defmt::info!("pin 31 set low; waiting ..");

    // just wait
    loop {
        continue;
    }
}
