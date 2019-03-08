use crate::percentage::Percentage;
use hal::gpio::{gpiob, Alternate, PushPull};
use hal::prelude::*;
use hal::pwm;
use hal::rcc;
use hal::stm32;

pub struct Motors {
    front_right_pwm_ch: pwm::Pwm<stm32::TIM4, pwm::C1>,
    front_left_pwm_ch: pwm::Pwm<stm32::TIM4, pwm::C2>,
    back_right_pwm_ch: pwm::Pwm<stm32::TIM4, pwm::C3>,
    back_left_pwm_ch: pwm::Pwm<stm32::TIM4, pwm::C4>,

    pub front_right: Percentage,
    pub front_left: Percentage,
    pub back_right: Percentage,
    pub back_left: Percentage,
}

impl Motors {
    pub fn new(
        pb6: gpiob::PB6<Alternate<PushPull>>,
        pb7: gpiob::PB7<Alternate<PushPull>>,
        pb8: gpiob::PB8<Alternate<PushPull>>,
        pb9: gpiob::PB9<Alternate<PushPull>>,
        apb1: &mut rcc::APB1,
        afio: &mut hal::afio::Parts,
        clocks: rcc::Clocks,
        tim4: stm32::TIM4,
    ) -> Motors {
        let (pwm_ch1, pwm_ch2, pwm_ch3, pwm_ch4) = tim4.pwm(
            (pb6, pb7, pb8, pb9),
            &mut afio.mapr,
            50.hz(),
            clocks,
            apb1,
        );

        Motors {
            front_right_pwm_ch: pwm_ch1,
            front_left_pwm_ch:  pwm_ch2,
            back_right_pwm_ch:  pwm_ch3,
            back_left_pwm_ch:   pwm_ch4,
            front_right: Percentage::new(0.0),
            front_left: Percentage::new(0.0),
            back_right: Percentage::new(0.0),
            back_left: Percentage::new(0.0),
        }
    }
}
