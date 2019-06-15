use crate::percentage::Percentage;
use hal::prelude::*;
use hal::pwm;
use hal::stm32;
use stm32::TIM4;

pub struct Motors {
    front_right: Percentage,
    front_left: Percentage,
    back_right: Percentage,
    back_left: Percentage,

    front_right_pwm_ch: pwm::Pwm<TIM4, pwm::C1>,
    front_left_pwm_ch: pwm::Pwm<TIM4, pwm::C2>,
    back_right_pwm_ch: pwm::Pwm<TIM4, pwm::C3>,
    back_left_pwm_ch: pwm::Pwm<TIM4, pwm::C4>,

    min_speed_duty: u16,
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

        let duty_1_ms = ch1.get_max_duty() / 20; // max duty = 20ms (50hz)
        let duty_1_ms_10_percent = (duty_1_ms * 10) / 100;

        Motors {
            front_right: Percentage::new(0.0),
            front_left: Percentage::new(0.0),
            back_right: Percentage::new(0.0),
            back_left: Percentage::new(0.0),
            front_right_pwm_ch: ch1,
            front_left_pwm_ch: ch2,
            back_right_pwm_ch: ch3,
            back_left_pwm_ch: ch4,

            // Duty cicles with a 10% margin on the limits
            min_speed_duty: duty_1_ms - duty_1_ms_10_percent,
            max_speed_duty: (2 * duty_1_ms) + duty_1_ms_10_percent,
        }
    }

    pub fn front_right(&mut self, percentage: Percentage) {
        self.front_right = percentage;
        let percent_duty = self.compute_duty_cicle(percentage);
        self.front_right_pwm_ch.set_duty(percent_duty);
    }

    pub fn get_front_right(&self) -> Percentage  {
        self.front_right
    }

    fn compute_duty_cicle(&self, percentage: Percentage) -> u16 {
        let duty_delta = self.max_speed_duty - self.min_speed_duty;
        let duty = ((percentage.value() / 100.0) * duty_delta as f32) + self.min_speed_duty as f32;
        duty as u16
    }
}
