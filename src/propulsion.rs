use crate::percentage::Percentage;
use hal::prelude::*;
use hal::pwm;
use hal::stm32;
use stm32::TIM4;

pub struct Motors {
    pub front_right: Percentage,
    pub front_left: Percentage,
    pub back_right: Percentage,
    pub back_left: Percentage,

    front_right_pwm_ch: pwm::Pwm<TIM4, pwm::C1>,
    front_left_pwm_ch: pwm::Pwm<TIM4, pwm::C2>,
    back_right_pwm_ch: pwm::Pwm<TIM4, pwm::C3>,
    back_left_pwm_ch: pwm::Pwm<TIM4, pwm::C4>,

    max_speed_duty: u16,
}

// FIXME: Use stm32f1xx_hal::pwm::Pins::Channels associated type instead of this
type PwmChannels = (
    pwm::Pwm<TIM4, pwm::C1>,
    pwm::Pwm<TIM4, pwm::C2>,
    pwm::Pwm<TIM4, pwm::C3>,
    pwm::Pwm<TIM4, pwm::C4>,
);

impl Motors {
    pub fn initialize((mut ch1, mut ch2, mut ch3, mut ch4): PwmChannels) -> Motors {
        ch1.set_duty(0);
        ch2.set_duty(0);
        ch3.set_duty(0);
        ch4.set_duty(0);

        ch1.enable();
        ch2.enable();
        ch3.enable();
        ch4.enable();

        let ch1_max_duty = ch1.get_max_duty();

        Motors {
            front_right: Percentage::new(0.0),
            front_left: Percentage::new(0.0),
            back_right: Percentage::new(0.0),
            back_left: Percentage::new(0.0),
            front_right_pwm_ch: ch1,
            front_left_pwm_ch: ch2,
            back_right_pwm_ch: ch3,
            back_left_pwm_ch: ch4,
            max_speed_duty: ch1_max_duty,
        }
    }
}

pub fn set(motors: &mut Motors) {
    let front_right_duty = percentage2duty(motors.max_speed_duty, &motors.front_right);
    let front_left_duty = percentage2duty(motors.max_speed_duty, &motors.front_left);
    let back_right_duty = percentage2duty(motors.max_speed_duty, &motors.back_right);
    let back_left_duty = percentage2duty(motors.max_speed_duty, &motors.back_left);

    motors.front_right_pwm_ch.set_duty(front_right_duty);
    motors.front_left_pwm_ch.set_duty(front_left_duty);
    motors.back_right_pwm_ch.set_duty(back_right_duty);
    motors.back_left_pwm_ch.set_duty(back_left_duty);
}

fn percentage2duty(max_duty: u16, percentage: &Percentage) -> u16 {
    let duty = ((percentage.value() / 100.0) * (max_duty as f32));
    duty as u16
}
