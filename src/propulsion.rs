use crate::percentage::Percentage;
use hal::prelude::*;
use hal::pwm;
use hal::stm32;
use stm32::TIM4;

pub type Throttle = Percentage;

pub enum Motor {
    FrontRight,
    FrontLeft,
    BackRight,
    BackLeft,
}

pub struct Motors {
    front_right: Throttle,
    front_left: Throttle,
    back_right: Throttle,
    back_left: Throttle,

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
            front_right: Throttle::new(0.0),
            front_left: Throttle::new(0.0),
            back_right: Throttle::new(0.0),
            back_left: Throttle::new(0.0),
            front_right_pwm_ch: ch1,
            front_left_pwm_ch: ch2,
            back_right_pwm_ch: ch3,
            back_left_pwm_ch: ch4,

            // Duty cicles with a 10% margin on the limits
            min_speed_duty: duty_1_ms - duty_1_ms_10_percent,
            max_speed_duty: (2 * duty_1_ms) + duty_1_ms_10_percent,
        }
    }

    pub fn get_motor_throttle(&self, motor: &Motor) -> Throttle {
        match motor {
            Motor::FrontRight => self.front_right,
            Motor::FrontLeft => self.front_left,
            Motor::BackRight => self.back_right,
            Motor::BackLeft => self.back_left,
        }
    }

    pub fn set_motor_throttle(&mut self, motor: &Motor, throttle: Throttle) {
        let throttle_duty_cicle = self.compute_duty_cicle(throttle);

        match motor {
            Motor::FrontRight => {
                self.front_right = throttle;
                self.front_right_pwm_ch.set_duty(throttle_duty_cicle);
            }

            Motor::FrontLeft => {
                self.front_left = throttle;
                self.front_left_pwm_ch.set_duty(throttle_duty_cicle);
            }

            Motor::BackRight => {
                self.back_right = throttle;
                self.back_right_pwm_ch.set_duty(throttle_duty_cicle);
            }

            Motor::BackLeft => {
                self.back_left = throttle;
                self.back_left_pwm_ch.set_duty(throttle_duty_cicle);
            }
        }
    }

    pub fn increase_motor_throttle(&mut self, motor: &Motor, percentage: Percentage) {
        let current_throttle = self.get_motor_throttle(motor);
        self.set_motor_throttle(motor, current_throttle + percentage);
    }

    pub fn decrease_motor_throttle(&mut self, motor: &Motor, percentage: Percentage) {
        let current_throttle = self.get_motor_throttle(motor);
        self.set_motor_throttle(motor, current_throttle - percentage);
    }

    fn compute_duty_cicle(&self, throttle: Throttle) -> u16 {
        let duty_delta = self.max_speed_duty - self.min_speed_duty;
        let duty = ((throttle.value() / 100.0) * duty_delta as f32) + self.min_speed_duty as f32;
        duty as u16
    }
}
