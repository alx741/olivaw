#![feature(proc_macro_hygiene)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32f1xx_hal as hal;

pub mod percentage;
pub mod propulsion;

// use cortex_m::asm;
use core::convert::Infallible;
use core::panic::PanicInfo;
use hal::prelude::*;
use hal::serial::Serial;
use hal::stm32;
use nb::block;
use percentage::Percentage;
use propulsion::{Motor, Motors};
use rt::{entry, exception, ExceptionFrame};
use stm32f1xx_hal::stm32::USART1;
// use stm32f1xx_hal::timer::Timer;
use ufmt::{uWrite, uwriteln};

struct TxUWriter {
    tx: hal::serial::Tx<USART1>,
}

impl uWrite for TxUWriter {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for c in s.chars() {
            block!(self.tx.write(c as u8)).ok();
            // block!(self.tx.write('R' as u8));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    // let pc = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    // let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

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

    let (tx, mut rx) = serial.split();

    let mut motors = Motors::initialize(tim4_pwm_channels);
    // let mut timer = Timer::syst(pc.SYST, 3.hz(), clocks);

    // asm::bkpt();
    let mut tx_uwriter = TxUWriter { tx: tx };
    loop {
        // led.set_low();
        let received = block!(rx.read()).unwrap();
        // led.set_high();

        match received as char {
            'j' => motors.decrease_motor_throttle(&Motor::FrontRight, Percentage::new(5.0)),
            'k' => motors.increase_motor_throttle(&Motor::FrontRight, Percentage::new(5.0)),
            _ => (),
        }

        uwriteln!(&mut tx_uwriter, "Throttle {}", motors.get_motor_throttle(&Motor::FrontRight).value() as u8).ok();
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
