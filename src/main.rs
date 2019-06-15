#![feature(proc_macro_hygiene)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32f1xx_hal as hal;
// extern crate void;

pub mod percentage;
pub mod propulsion;

// use cortex_m::asm;
use core::panic::PanicInfo;
use hal::prelude::*;
use hal::stm32;
use hal::serial::{Serial};
use nb::block;
#[allow(unused_imports)]
use ufmt::{uwriteln, uWrite};
#[allow(unused_imports)]
use percentage::Percentage;
use propulsion::Motors;
use rt::{entry, exception, ExceptionFrame};
use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::stm32::USART1;
// use void::Void;

struct TxUWriter {
    tx: hal::serial::Tx<USART1>,
}

impl uWrite for TxUWriter {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for c in s.chars() {
            block!(self.tx.write(c as u8));
            // block!(self.tx.write('R' as u8));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let pc = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

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

    // USART1
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;

    let serial = Serial::usart1(
        p.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    // separate into tx and rx channels
    let (mut tx, mut rx) = serial.split();

    let mut motors = Motors::initialize(tim4_pwm_channels);
    let mut timer = Timer::syst(pc.SYST, 3.hz(), clocks);

    led.set_low();
    motors.front_right = Percentage::new(0.0);
    motors.back_left = Percentage::new(0.0);
    propulsion::set(&mut motors);
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    led.set_high();

    // asm::bkpt();
    let mut throttle = 0.0;
    let mut txUWriter = TxUWriter { tx: tx};
    loop {
        uwriteln!(&mut txUWriter, "test {}", 1);
        let received = block!(rx.read()).unwrap();
        block!(timer.wait()).unwrap();

        motors.front_right = Percentage::new(throttle);
        motors.back_left = Percentage::new(throttle);
        propulsion::set(&mut motors);
        block!(timer.wait()).unwrap();

        if throttle == 100.0 {
            throttle = 0.0;
        } else {
            throttle += 20.0;
        }
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

#[exception]
fn DefaultHandler(_irqn: i16) {
    loop {}
}
