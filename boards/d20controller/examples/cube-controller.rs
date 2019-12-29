#![no_std]
#![no_main]

extern crate d20controller as hal;
extern crate panic_halt;
//extern crate panic_semihosting;

use crate::hal::pac::gclk::genctrl::SRC_A::DPLL0;
use crate::hal::pac::gclk::pchctrl::GEN_A::GCLK1;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::sercom::PadPin;
use hal::prelude::*;
//use hal::watchdog::{Watchdog, WatchdogTimeout};
use nb::block;
use hal::dma;


mod gfx;

use crate::hal::timer::TimerCounter;

const BUFFER_LENGTH: usize = (32*3*3)*(24)*(2);

/* ice40 connections */
type ice_spi = hal::sercom::SPIMaster0<
            hal::sercom::Sercom0Pad3<hal::gpio::Pa7<hal::gpio::PfD>>,
            hal::sercom::Sercom0Pad0<hal::gpio::Pa4<hal::gpio::PfD>>,
            hal::sercom::Sercom0Pad1<hal::gpio::Pa5<hal::gpio::PfD>>,>; 
type ice_cs = hal::gpio::Pa6<hal::gpio::Output<hal::gpio::PushPull>>;
type ice_io = hal::gpio::Pa2<hal::gpio::Input<hal::gpio::PullUp>>;

/* bmx160 IMU */
type IMU_SPI = hal::sercom::SPIMaster1<
            hal::sercom::Sercom1Pad3<hal::gpio::Pa19<hal::gpio::PfC>>,
            hal::sercom::Sercom1Pad0<hal::gpio::Pa16<hal::gpio::PfC>>, 
            hal::sercom::Sercom1Pad1<hal::gpio::Pa17<hal::gpio::PfC>>,>;
type IMU_CS = hal::gpio::Pa18<hal::gpio::Output<hal::gpio::PushPull>>;

pub fn matrix_size() -> (u16, u16) {
    (32*3, 24*2)
}


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);

    delay.delay_ms(100u16);
    //let wdt = Watchdog::new_with_timeout(peripherals.WDT, WatchdogTimeout::Timeout256ms);

    let gclk = clocks.gclk0();

    // output a 12MHz clock signal to the ice40.
    // Due to hardware constraints we are locked into using GCLK1 for this task.
    let _gclk1 = clocks
        .configure_gclk_divider_and_source(GCLK1, 4, DPLL0, true)
        .unwrap();
    pins.ice40_gclk.into_function_m(&mut pins.port);

    // On board LEDs used for status indication.
    let mut red_led = pins.led_r.into_open_drain_output(&mut pins.port);
    let mut green_led = pins.led_g.into_open_drain_output(&mut pins.port);
    let mut blue_led = pins.led_b.into_open_drain_output(&mut pins.port);

    // Clear LEDs on power up
    red_led.set_high().unwrap();
    green_led.set_high().unwrap();
    blue_led.set_high().unwrap();

    // Init SPI for connection to the ice40 
    let spi_clk = &clocks.sercom0_core(&gclk).unwrap();
    let mut spi: ice_spi = hal::sercom::SPIMaster0::new(
        spi_clk,
        60_000_000u32.hz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,  
        },
        peripherals.SERCOM0,
        &mut peripherals.MCLK,
        (pins.ice40_miso.into_pad(&mut pins.port), pins.ice40_mosi.into_pad(&mut pins.port), pins.ice40_sck.into_pad(&mut pins.port)),
    );

    // Sideband connection for the ice40
    let mut ice40_cs = pins.ice40_ss.into_push_pull_output(&mut pins.port);
    let mut ice40_rst = pins.ice40_reset.into_push_pull_output(&mut pins.port);
    let ice40_io0 = pins.ice40_io0.into_pull_up_input(&mut pins.port);
    let ice40_done = pins.ice40_cdone.into_pull_up_input(&mut pins.port);


    let button = pins.button.into_pull_up_input(&mut pins.port);


    let imu_clk = &clocks.sercom1_core(&gclk).unwrap();
    let mut imu_spi: IMU_SPI = hal::sercom::SPIMaster1::new(
        imu_clk,
        20_000_000u32.hz(),
        embedded_hal::spi::Mode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,  
        },
        peripherals.SERCOM1,
        &mut peripherals.MCLK,
        (pins.imu_miso.into_pad(&mut pins.port), pins.imu_mosi.into_pad(&mut pins.port), pins.imu_sck.into_pad(&mut pins.port)),
    );
    let mut imu_cs = pins.imu_ss.into_push_pull_output(&mut pins.port);
    

    // Perform the reset sequence to place ice40 into SPI config mode
    ice40_rst.set_low().unwrap();
    ice40_cs.set_low().unwrap();
    delay.delay_ms(2u8);
    ice40_rst.set_high().unwrap();
    delay.delay_ms(4u8);


    
    imu_cs.set_high().unwrap();
    delay.delay_ms(10u8);
    imu_cs.set_low().unwrap();
    delay.delay_ms(10u8);
    block!(imu_spi.send(0x80 | 0x7F)).unwrap();
    block!(imu_spi.send(0)).unwrap();
    delay.delay_ms(10u8);
    imu_cs.set_high().unwrap();

    // Transmit a Bitstream to the ice40
    //let ice40_bin = include_bytes!("../ice40.bin");
    //let ice40_bin = include_bytes!("../../../../panel_tests/projects/rgb_panel/build-tmp/rgb_panel.bin");
    
    let ice40_bin = include_bytes!("../../../../GlassUnicorn/gateware/cube-led-drive/projects/rgb_panel/build-tmp/rgb_panel.bin");

    /* Transmit using a Loop */
    //ice40_loop(&mut spi, ice40_bin);

    let mut buffer: [u8;BUFFER_LENGTH] = [0; BUFFER_LENGTH];

    //block!(spi.send(0)).unwrap();

    // Transfer using DMA 
    let mut dmac_sram: dma::DescriptorList = Default::default();
    
    let dmac: dma::Controller = dma::Controller::new(
        peripherals.DMAC,
        &mut peripherals.MCLK,
        &mut core.NVIC,
        &dmac_sram
    ).unwrap();
    

    let mut ch0: dma::Channel = dma::Channel::new(
        &dmac,
        0,
        &dmac_sram
    ).unwrap();
    
    ch0.add_descriptor(
        ice40_bin.as_ptr(),
        0x40003028u32 as *mut u8,
        52045,
        //ice40_bin.len() as u16,
        hal::dma::controller::BeatSize::Byte,
        true,
        false,
        0,
        false
    );

    ch0.set_trigger(hal::pac::dmac::chctrla::TRIGSRC_A::SERCOM_0_TX);
    ch0.set_action(hal::pac::dmac::chctrla::TRIGACT_A::BURST);
    
    /* Start transfer of data*/
    ch0.start_job().unwrap();
    while(!ch0.is_complete() || ch0.is_pending()){ }
    
    ch0.change_descriptor((ice40_bin.as_ptr() as *const u8 as u32 + 52045) as *const u8, 0 as *mut u8, 52045);
    ch0.start_job().unwrap();
    while(!ch0.is_complete() || ch0.is_pending()){}
    
    ch0.change_descriptor(ice40_bin.as_ptr() as *const u8, 0 as *mut u8, 100);
    ch0.start_job().unwrap();
    while(!ch0.is_complete() || ch0.is_pending()){ }
   

    let timer_clock = clocks.tc2_tc3(&gclk).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(60u32.hz());

    ice40_cs.set_high().unwrap();


    imu_cs.set_low().unwrap();
    delay.delay_ms(1u8);
    block!(imu_spi.send(0x40)).unwrap();
    block!(imu_spi.send(0b00101011)).unwrap();
    delay.delay_ms(1u8);
    imu_cs.set_high().unwrap();
    delay.delay_ms(1u8);


    imu_cs.set_low().unwrap();
    delay.delay_ms(1u8);
    block!(imu_spi.send(0x7E)).unwrap();
    block!(imu_spi.send(0x12)).unwrap();
    delay.delay_ms(1u8);
    imu_cs.set_high().unwrap();
    delay.delay_ms(200u8);

//    imu_cs.set_low().unwrap();
//    delay.delay_ms(1u8);
//    block!(imu_spi.send(0x7E)).unwrap();
//    block!(imu_spi.send(0x17)).unwrap();
//    delay.delay_ms(1u8);
//    imu_cs.set_high().unwrap();
//    delay.delay_ms(200u8);
//
//    imu_cs.set_low().unwrap();
//    delay.delay_ms(1u8);
//    block!(imu_spi.send(0x7E)).unwrap();
//    block!(imu_spi.send(0x19)).unwrap();
//    delay.delay_ms(1u8);
//    imu_cs.set_high().unwrap();
//    delay.delay_ms(200u8);
    
    let mut imu_x : i16 = 0;
    let mut imu_y : i16 = 0;
    let mut imu_z : i16 = 0;
    
    let mut last_states: u8 = 0;
    let mut current_animation: u8 = 0;
    loop {
        
        //delay.delay_ms(1000u16);
        block!(timer.wait()).ok();

        let mut imu: [u8;12] = [0; 12];

        //delay.delay_ms(50u8);

        /* Read from the IMU */
        unsafe{
        imu_cs.set_low().unwrap();
        block!(imu_spi.send(0x80 | 0x12)).unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[0] = imu_spi.read().unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[1] = imu_spi.read().unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[2] = imu_spi.read().unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[3] = imu_spi.read().unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[4] = imu_spi.read().unwrap();
        block!(imu_spi.send(0x00)).unwrap();
        imu[5] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[6] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[7] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[8] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[9] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[10] = imu_spi.read().unwrap();
//        block!(imu_spi.send(0x00)).unwrap();
//        imu[11] = imu_spi.read().unwrap();
        imu_cs.set_high().unwrap();
        }

        
        imu_x = (((imu[1] as u16) << 8) | imu[0] as u16) as i16;
        imu_y = (((imu[3] as u16) << 8) | imu[2] as u16) as i16;
        imu_z = (((imu[5] as u16) << 8) | imu[4] as u16) as i16;
        
        
        //current_animation = 1;
//
        //if(imu_x != 0 && imu_x != 0xFF)
        //{
        //    current_animation = 0;
        //}

        match current_animation {
            2 => gfx::gfx_basic_pattern::draw(&mut buffer),
            1 => gfx::gfx_rainbow::draw(&mut buffer),
            3 => gfx::gfx_twinkle::draw(&mut buffer),
            5 => gfx::gfx_sine_rainbow_001::draw(&mut buffer),
            4 => gfx::gfx_sine_rainbow::draw(&mut buffer),
            6 => gfx::gfx_sine_rainbow_imu::draw(&mut buffer, imu_x, imu_y, imu_z),
            0 => gfx::gfx_sine_rainbow_imu_001::draw(&mut buffer, imu_x, imu_y, imu_z),
            default => gfx::gfx_basic_pattern::draw(&mut buffer),
        }
        
        
        green_led.toggle();
        transmit_dma(&mut spi, &mut ch0, &mut ice40_cs, &ice40_io0, &mut delay, &buffer);

        // Very basic logic, if the button is pressed then switch to a new animation 
        if button.is_low().unwrap() & (last_states == 0) {
            current_animation += 1;

            if current_animation > 4 {
                current_animation = 0;
            }
        }

        last_states <<= 1;
        last_states |= if button.is_low().unwrap() {1} else {0};
    }
}


fn delay_wait(ice40_io0: &ice_io,
                delay: &mut Delay,){
        //unsafe {
        delay.delay_us(2u8);
        let mut timeout: u8 = 0u8;
        while ice40_io0.is_low().unwrap() && timeout < 100 {
            delay.delay_us(1u8);
            timeout += 1;
        }
        delay.delay_us(2u8);
}

fn transmit_dma(spi: &mut ice_spi, dma: &mut dma::Channel,
                 ice40_cs: &mut ice_cs, ice40_io0: &ice_io,
                delay: &mut Delay, buffer: &[u8]){

    //delay.delay_ms(1u8);
    let mut p: u32 = buffer.as_ptr() as *const u8 as u32;
    for l in 0..(24*2) {


    // Timeout used on first pass as this signal is not active.
    delay_wait(ice40_io0, delay);
    
    ice40_cs.set_low().unwrap();
    
    delay.delay_us(1u8);
    block!(spi.send(0x80)).unwrap();

    dma.change_descriptor(p as *const u8, 0 as *mut u8, 32*3*3);
    dma.start_job().unwrap();
    while !dma.is_complete() || dma.is_pending() { }
    
    //delay.delay_us(1u8);
    ice40_cs.set_high().unwrap();

    p += 32*3*3;
    delay.delay_us(8u8);
    

    ice40_cs.set_low().unwrap();
    delay.delay_us(1u8);
    block!(spi.send(0x03)).unwrap();
    let line: u8 = if l >= 24 {l+8} else {l};
    block!(spi.send(line)).unwrap();
    delay.delay_us(1u8);
    ice40_cs.set_high().unwrap();
    
    delay.delay_us(5u8);
    delay_wait(ice40_io0, delay);
    
    

    }


    delay.delay_us(5u16);
    ice40_cs.set_low().unwrap();
    block!(spi.send(0x04)).unwrap();
    block!(spi.send(0)).unwrap();
    
    ice40_cs.set_low().unwrap();
    ice40_cs.set_low().unwrap();
    ice40_cs.set_high().unwrap();

    let mut wait: bool = true;

    while wait == true {
        ice40_cs.set_low().unwrap();
        block!(spi.send(0x00)).unwrap();
        let r = block!(spi.read()).unwrap();
        block!(spi.send(0x00)).unwrap();
        
        delay.delay_us(1u8);

        ice40_cs.set_high().unwrap();

        wait = (r & 0x02 != 0);
    }
}
