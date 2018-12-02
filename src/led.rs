use wiringpi::pin::{Gpio, SoftPwmPin};
use wiringpi::WiringPi;

pub struct Led {
    pin: SoftPwmPin<Gpio>,
}

impl Led {
    pub fn new(pin_id: u16, pi: &WiringPi<Gpio>) -> Led {
        Led {
            pin: pi.soft_pwm_pin(pin_id),
        }
    }

    pub fn update(&self, level: u8) {
        self.pin.pwm_write(level as i32);
    }
}
