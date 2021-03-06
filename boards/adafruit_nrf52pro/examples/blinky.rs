#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nb::block;

#[allow(unused_imports)]
use panic_semihosting;

use adafruit_nrf52pro_bsc::hal::{
    prelude::*,
    gpio::{p0, Level},
    timer::{self, Timer},
};
use adafruit_nrf52pro_bsc::nrf52832_pac::{Peripherals};
use adafruit_nrf52pro_bsc::Pins;


#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(p.P0));

    let mut led1 = pins.led1.into_push_pull_output(Level::Low);
    let mut led2 = pins.led2.into_push_pull_output(Level::Low);

    let mut timer = Timer::new(p.TIMER0);

    // Alternately flash the red and blue leds
    loop {
        led1.set_low();
        led2.set_high();
        delay(&mut timer, 250_000); // 250ms
        led1.set_high();
        led2.set_low();
        delay(&mut timer, 1_000_000); // 1s
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    block!(timer.wait());
}
