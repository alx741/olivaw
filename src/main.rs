#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

pub mod percentage;
pub mod propulsion;

// use cortex_m::asm;
// use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::stm32;
use nb::block;
use percentage::Percentage;
use propulsion::Motors;
use rt::{entry, exception, ExceptionFrame};
use stm32f1xx_hal::timer::Timer;

#[entry]
fn main() -> ! {
    let pc = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
    gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_low();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    // TIM4
    let pb6 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let pb7 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let pb8 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let pb9 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let tim4_pwm_channels = p.TIM4.pwm(
        (pb6, pb7, pb8, pb9),
        &mut afio.mapr,
        50.hz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut motors = Motors::initialize(tim4_pwm_channels);

    let mut timer = Timer::syst(pc.SYST, 1.hz(), clocks);

    // asm::bkpt();
    let mut throttle = 0.0;
    loop {
        block!(timer.wait()).unwrap();
        motors.front_right = Percentage::new(throttle);
        motors.back_left = Percentage::new(throttle);
        propulsion::set(&mut motors);

        if throttle == 100.0 {
            throttle = 0.0;
        } else {
            throttle += 10.0;
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    // panic!("{:#?}", ef);
    loop {}
}

#[exception]
fn DefaultHandler(irqn: i16) {
    // panic!("Unhandled exception (IRQn = {})", irqn);
    loop {}
}
