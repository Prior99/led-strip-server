use color::Color;
use led::Led;
use wiringpi::pin::Gpio;
use wiringpi::WiringPi;

const PIN_RED: u16 = 22;
const PIN_GREEN: u16 = 17;
const PIN_BLUE: u16 = 24;

pub struct Leds {
    r: Led,
    g: Led,
    b: Led,
    pub state: Color,
}

impl Leds {
    pub fn new(pi: &WiringPi<Gpio>) -> Leds {
        let mut leds = Leds {
            r: Led::new(PIN_RED, &pi),
            g: Led::new(PIN_GREEN, &pi),
            b: Led::new(PIN_BLUE, &pi),
            state: Color::new(0, 0, 0),
        };
        leds.update(Color::new(0, 0, 0));
        leds
    }

    pub fn update(&mut self, color: Color) {
        self.r.update(color.r);
        self.g.update(color.g);
        self.b.update(color.b);
        self.state = color;
    }
}
