#![deny(unsafe_code)]
// #![deny(warnings)]
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
use percentage::Percentage;
use propulsion::Motors;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    // let mut motors = Motors::new();
    // motors.front_right = Percentage::new(50.5);
    // hprintln!("motors: {:#?}", motors).unwrap();

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
    // .0;

    let mut motors = Motors::new(tim4_pwm_channels);


    // // let max = pwm.get_max_duty();

    // // // hprintln!("max duty: {}", max).unwrap();
    // _pwm_ch1.enable();
    // // // asm::bkpt();

    // // full
    // // _pwm_ch1.set_duty(500.hz());
    // // pwm.set_duty((max*10)/100);
    // // asm::bkpt();

    // // // dim
    // // // pwm.set_duty(4266);
    // // pwm.set_duty((max*5)/100);
    // // asm::bkpt();

    // // // zero
    // // // pwm.set_duty(2666);
    // // pwm.set_duty((max*8)/100);
    // // asm::bkpt();

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
