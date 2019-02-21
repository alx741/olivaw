#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use hal::prelude::*;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);

    // #[cfg(feature = "stm32f103")]
    gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_low();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
