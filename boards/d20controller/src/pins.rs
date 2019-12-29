//! ItsyBitsy M4 Express pins

use super::{hal, pac::MCLK, pac::SERCOM0, pac::SERCOM2, pac::SERCOM3, target_device};

use embedded_hal::timer::{CountDown, Periodic};
use hal::define_pins;
use hal::gpio::{self, *};
use hal::sercom::{I2CMaster2, PadPin, SPIMaster1, Sercom2Pad0, Sercom2Pad1, UART3};
use hal::time::Hertz;

use hal::clock::GenericClockController;
use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};


#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin led_r = a13,
    pin led_g = a15,
    pin led_b = a14,

    pin ice40_miso = a7,
    pin ice40_mosi = a4,
    pin ice40_sck = a5,
    pin ice40_ss = a6,
    pin ice40_cdone = b8,
    pin ice40_reset = b9,
    pin ice40_io0 = a2,

    pin ice40_gclk = b23,

    pin button = b22,

    pin imu_miso = a19,
    pin imu_mosi = a16,
    pin imu_sck = a17,

    pin imu_ss = a18,

);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        Sets {
            port: self.port,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Port
    pub port: Port,
}
