//! A rainbow-LED example application
//! This example uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate launchpad;
extern crate alloc;
#[macro_use]
extern crate collections;
extern crate embedded_serial;

use core::fmt::Write;
use embedded_serial::NonBlockingRx;
use launchpad::cpu::{gpio, systick, timer, uart};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

#[no_mangle]
pub extern "C" fn main() {
    gpio::set_direction(gpio::PinPort::PortF(gpio::Pin::Pin4),
                        gpio::PinMode::Output);
    //gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin4));
    gpio::set(gpio::PinPort::PortF(gpio::Pin::Pin4), gpio::Level::High);
    loop {}


    //let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    /*
    let mut loops = 0;
    let mut ticks_last = systick::SYSTICK_MAX;
    let mut tr = timer::Timer::new(timer::TimerId::Timer0B);
    let mut tb = timer::Timer::new(timer::TimerId::Timer1A);
    let mut tg = timer::Timer::new(timer::TimerId::Timer1B);
    tr.enable_pwm(255);
    tb.enable_pwm(255);
    // Green is a bit bright! Tone it down.
    tg.enable_pwm(512);
    gpio::set_direction(gpio::PinPort::PortF(gpio::Pin::Pin1),
                        gpio::PinMode::Peripheral);
    gpio::set_direction(gpio::PinPort::PortF(gpio::Pin::Pin2),
                        gpio::PinMode::Peripheral);
    gpio::set_direction(gpio::PinPort::PortF(gpio::Pin::Pin3),
                        gpio::PinMode::Peripheral);
    gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin1));
    gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin2));
    gpio::enable_ccp(gpio::PinPort::PortF(gpio::Pin::Pin3));
    let mut angle = 0;
    loop {
        let (r, g, b) = calculate_rgb(angle);
        tr.set_pwm(r as u32);
        tb.set_pwm(b as u32);
        tg.set_pwm(g as u32);
        // while let Ok(ch) = uart.getc_try() {
        //     writeln!(uart, "byte read {}", ch).unwrap();
        // }
        loops = loops + 1;
        angle = angle + 5;
        if angle >= 360 {
            angle -= 360;
            let delta = systick::get_since(ticks_last);
            ticks_last = systick::get_ticks();
            // writeln!(uart,
            //          "Hello, world! Loops = {}, elapsed = {}, run_time = {}",
            //          loops,
            //          systick::ticks_to_usecs(delta),
            //          systick::run_time_us() as u32)
            //     .unwrap();
        };
        launchpad::delay(50);
    }
    */
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn calculate_rgb(angle: u16) -> (u8, u8, u8) {
    let angle = angle % 360;

    let g = match angle {
        x if x < 60 => ((x as f32) - 0.0) / 60.0,
        x if x < 180 => 1.0,
        x if x < 240 => 1.0 - (((x as f32) - 180.0) / 60.0),
        _ => 0.0
    };

    let b = match angle {
        x if x < 120 => 0.0,
        x if x < 180 => ((x as f32) - 120.0) / 60.0,
        x if x < 300 => 1.0,
        x => 1.0 - (((x as f32) - 300.0) / 60.0),
    };

    let r = match angle {
        x if x < 60 => 1.0,
        x if x < 120 => 1.0 - (((x as f32) - 60.0) / 60.0),
        x if x < 240 => 0.0,
        x if x < 300 => ((x as f32) - 240.0) / 60.0,
        _ => 1.0,
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
