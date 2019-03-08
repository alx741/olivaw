use crate::percentage::Percentage;
use hal::gpio::{gpiob, Alternate, PushPull};
use hal::prelude::*;
use hal::pwm;
use hal::rcc;
use hal::stm32;
use stm32::TIM4;

pub struct Motors {
    front_right_pwm_ch: pwm::Pwm<TIM4, pwm::C1>,
    front_left_pwm_ch: pwm::Pwm<TIM4, pwm::C2>,
    back_right_pwm_ch: pwm::Pwm<TIM4, pwm::C3>,
    back_left_pwm_ch: pwm::Pwm<TIM4, pwm::C4>,

    pub front_right: Percentage,
    pub front_left: Percentage,
    pub back_right: Percentage,
    pub back_left: Percentage,
}

// FIXME: Use stm32f1xx_hal::pwm::Pins::Channels associated type instead of this
type PwmChannels = (
    pwm::Pwm<TIM4, pwm::C1>,
    pwm::Pwm<TIM4, pwm::C2>,
    pwm::Pwm<TIM4, pwm::C3>,
    pwm::Pwm<TIM4, pwm::C4>,
);

impl Motors {
    pub fn new((ch1, ch2, ch3, ch4): PwmChannels) -> Motors {
        Motors {
            front_right_pwm_ch: ch1,
            front_left_pwm_ch: ch2,
            back_right_pwm_ch: ch3,
            back_left_pwm_ch: ch4,
            front_right: Percentage::new(0.0),
            front_left: Percentage::new(0.0),
            back_right: Percentage::new(0.0),
            back_left: Percentage::new(0.0),
        }
    }
}
