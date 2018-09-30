#![feature(drain_filter)]
#[macro_use]
extern crate clap;
extern crate wiringpi;
extern crate ws;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate simplelog;

mod color;
mod led;
mod leds;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use ws::{Message, Sender, WebSocket};
use color::Color;
use leds::Leds;

fn message_to_color(message: Message) -> Color {
    match message {
        Message::Text(json) => {
            match serde_json::from_str(&json) {
                Err(_) => Color::new(0, 0, 0),
                Ok(value) => value,
            }
        },
        Message::Binary (bin) => {
            if bin.len() != 3 {
                Color::new(0, 0, 0)
            } else {
                Color::new(bin[0], bin[1], bin[2])
            }
        },
    }
}

fn main() {
    use clap::App;
    let yml = load_yaml!("commandline.yml");
    let matches = App::from_yaml(yml).get_matches();
    let log_level = if matches.is_present("verbose") {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };
    if let Err(log_error) = simplelog::TermLogger::init(log_level, simplelog::Config::default()) {
        println!("Couldn't setup logging: {}", log_error);
    }
    match matches.subcommand() {
        ("start", Some(start_matches)) => {
            let port = start_matches.value_of("port").expect("No port specified.");
            let host = start_matches.value_of("host").expect("No host specified.");
            let address = format!("{}:{}", host, port);

            let pi = wiringpi::setup_gpio();
            let leds = Arc::new(Mutex::new(Leds::new(&pi)));

            info!("Listening for new websocket connections on {}", address);
            let mut broadcaster: Arc<RefCell<Option<Sender>>> = Arc::new(RefCell::new(None));
            let server_socket = WebSocket::new(|_| {
                |msg| {
                    let color = message_to_color(msg);
                    let json = serde_json::to_string(&color).unwrap();
                    leds.lock().unwrap().update(color);
                    let broadcaster_arc = broadcaster.clone();
                    if let Some(ref local_broadcaster) = *broadcaster_arc.borrow() {
                        if let Err(error) = local_broadcaster.send(Message::text(json)) {
                            warn!("Failed to send message to client: {:?}", error);
                        }
                    }
                    Ok(())
                }
            }).expect("Unable to create websocket.");
            broadcaster.replace(Some(server_socket.broadcaster()));
            if let Err(error) = server_socket.listen(address.clone()) {
                error!("Error opening socket on {}: {:?}", address, error);
            };
        },
        ("", None) => println!("Unkown command"),
        _ => unreachable!(),
    }
}
