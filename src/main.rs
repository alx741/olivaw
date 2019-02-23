#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

// use cortex_m_semihosting::hprintln;

// use cortex_m::asm;
use hal::prelude::*;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);

    // #[cfg(feature = "stm32f103")]
    gpioc.pc13.into_push_pull_output(&mut gpioc.crh).set_low();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    // TIM4
    let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let (mut _pwm_ch1, mut _pwm_ch2, mut _pwm_ch3, mut _pwm_ch4) = p
        .TIM4
        .pwm(
            (c1, c2, c3, c4),
            &mut afio.mapr,
            50.hz(),
            clocks,
            &mut rcc.apb1,
        );
        // .0;

    // let max = pwm.get_max_duty();

    // // hprintln!("max duty: {}", max).unwrap();
    _pwm_ch1.enable();
    // // asm::bkpt();

    // full
    _pwm_ch1.set_duty(500.hz());
    // pwm.set_duty((max*10)/100);
    // asm::bkpt();

    // // dim
    // // pwm.set_duty(4266);
    // pwm.set_duty((max*5)/100);
    // asm::bkpt();

    // // zero
    // // pwm.set_duty(2666);
    // pwm.set_duty((max*8)/100);
    // asm::bkpt();

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
