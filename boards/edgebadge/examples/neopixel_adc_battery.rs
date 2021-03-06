//! Display battery percentage on the neopixels.

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;

use edgebadge as hal;

use hal::adc::Adc;
use hal::entry;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::hsv::RGB8;
use smart_leds::{brightness, SmartLedsWrite};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();

    let mut adc0 = Adc::adc0(peripherals.ADC0, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut battery = pins.battery.init(&mut pins.port);

    // neopixels
    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    //todo put this on a .. 10minute, 30min, update timer
    loop {
        let battery_data = battery.read(&mut adc0);

        let mut colors = [
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
        ];

        if battery_data < 3.6 {
            colors[0] = RGB8::from((255, 0, 0));
        } else if (battery_data >= 3.6) && (battery_data < 3.8) {
            colors[0] = RGB8::from((255, 0, 0));
            colors[1] = RGB8::from((255, 0, 0));
        } else if (battery_data >= 3.8) && (battery_data < 3.9) {
            colors[0] = RGB8::from((255, 255, 0));
            colors[1] = RGB8::from((255, 255, 0));
            colors[2] = RGB8::from((255, 255, 0));
        } else if (battery_data >= 3.9) && (battery_data < 4.0) {
            colors[0] = RGB8::from((0, 255, 0));
            colors[1] = RGB8::from((0, 255, 0));
            colors[2] = RGB8::from((0, 255, 0));
            colors[3] = RGB8::from((0, 255, 0));
        } else {
            colors[0] = RGB8::from((0, 255, 0));
            colors[1] = RGB8::from((0, 255, 0));
            colors[2] = RGB8::from((0, 255, 0));
            colors[3] = RGB8::from((0, 255, 0));
            colors[4] = RGB8::from((0, 255, 0));
        };

        neopixel
            .write(brightness(colors.iter().cloned(), 1))
            .unwrap();
        delay.delay_ms(5u8);

        delay.delay_ms(10u8);
    }
}
