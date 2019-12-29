#![no_std]
#![recursion_limit = "1024"]

pub mod pins;

use atsamd_hal as hal;

use hal::*;

pub use hal::common::*;
pub use hal::samd51::*;
pub use hal::target_device as pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;
pub use pins::Pins;

use embedded_hal::timer::{CountDown, Periodic};
use gpio::{PfC, Port};
use hal::clock::GenericClockController;
use hal::gpio::*;
use hal::time::Hertz;

#[cfg(feature = "use_uart_debug")]
pub use hal::dbgprint;

