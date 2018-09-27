extern crate wiringpi;

use std::net::UdpSocket;
use std::io::Result;
use wiringpi::pin::{SoftPwmPin,Gpio};
use wiringpi::WiringPi;

const PIN_RED: u16 = 22;
const PIN_GREEN: u16 = 17;
const PIN_BLUE: u16 = 24;

pub struct Led {
    pin: SoftPwmPin<Gpio>,
}

impl Led {
    pub fn new(pin_id: u16, pi: &WiringPi<Gpio>) -> Led {
        Led {
            pin: pi.soft_pwm_pin(pin_id)
        }
    }

    pub fn update(&mut self, level: u8) {
        self.pin.pwm_write(level as i32);
    }
}

fn main() -> Result<()> {
    let pi = wiringpi::setup_gpio();
    let mut leds = [Led::new(PIN_RED, &pi), Led::new(PIN_GREEN, &pi), Led::new(PIN_BLUE, &pi)];
    let socket = UdpSocket::bind("0.0.0.0:7305").expect("Couldn't bind to 0.0.0.0:7305");
    loop {
        let mut rgb: [u8; 3] = [0; 3];
        socket.recv_from(&mut rgb)?;
        rgb.iter().enumerate().for_each(|(index, value)| leds[index].update(value.clone()))
    }
}
