#![no_main]
#![no_std]

use lpc_assistant as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = lpc8xx_hal::Peripherals::take().unwrap();
    let mut syscon = p.SYSCON.split();
    let gpio = p.GPIO.enable(&mut syscon.handle);
    let pin31 = p.pins.pio1_0.into_input_pin(gpio.tokens.pio1_0);

    defmt::info!("is pin 31 low (0V)?");
    assert!(pin31.is_low(), "pin 31 is not low");
    defmt::info!("pin 31 IS low (0V)");

    lpc_assistant::exit()
}
