#[macro_use]
extern crate clap;
extern crate wiringpi;
extern crate ws;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use wiringpi::pin::{SoftPwmPin, Gpio};
use wiringpi::WiringPi;
use ws::{listen, Message};

const PIN_RED: u16 = 22;
const PIN_GREEN: u16 = 17;
const PIN_BLUE: u16 = 24;

#[derive(Deserialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

pub struct Led {
    pin: SoftPwmPin<Gpio>,
}

impl Led {
    pub fn new(pin_id: u16, pi: &WiringPi<Gpio>) -> Led {
        Led {
            pin: pi.soft_pwm_pin(pin_id)
        }
    }

    pub fn update(&self, level: u8) {
        self.pin.pwm_write(level as i32);
    }
}

fn main() {
    use clap::App;
    let yml = load_yaml!("commandline.yml");
    let matches = App::from_yaml(yml).get_matches();
    match matches.subcommand() {
        ("start", Some(start_matches)) => {
            let port = start_matches.value_of("port").expect("No port specified.");
            let host = start_matches.value_of("host").expect("No host specified.");
            let pi = wiringpi::setup_gpio();
            let leds = [Led::new(PIN_RED, &pi), Led::new(PIN_GREEN, &pi), Led::new(PIN_BLUE, &pi)];
            let address = format!("{}:{}", host, port);
            println!("Listening for new websocket connections on {}", address);
            if let Err(error) = listen(address.clone(), |_| {
                |msg| {
                    println!("{:?}", msg);
                    let (r, g, b) = match msg {
                        Message::Text(json) => {
                            let Color { r, g, b } = match serde_json::from_str(&json) {
                                Err(_) => Color { r: 0, g: 0, b: 0 },
                                Ok(value) => value
                            };
                            (r, g, b)
                        },
                        Message::Binary (bin) => {
                            if bin.len() != 3 {
                                (0, 0, 0)
                            } else {
                                (bin[0], bin[1], bin[2])
                            }
                        },
                    };
                    println!("Received RGB {}, {}, {}", r, g, b);
                    [r, g, b].iter().enumerate().for_each(|(index, value)| leds[index].update(value.clone()));
                    Ok(())
                }
            }) {
                println!("Error opening socket on {}: {:?}", address, error);
            };
        },
        ("", None) => println!("Unkown command"),
        _ => unreachable!(),
    }
}
