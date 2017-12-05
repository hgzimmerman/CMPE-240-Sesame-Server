#![feature(plugin)]
#![plugin(rocket_codegen)]
//#![feature(drop_types_in_const)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
//#![feature(const_fn)]
#![feature(duration_from_micros)]


extern crate sysfs_gpio;



extern crate rocket;
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;



use rocket_contrib::{Json, Value};
use rocket::State;



const SERVO_GPIO_PIN: u32 = 18;

const ROTATE_CLOCKWISE_DELAY:u32 = 1;
const ROTATE_COUNTERCLOCKWISE_DELAY: u32 = 2;
const STANDARD_DELAY: u32 = 1;
const DISTANCE_ITERATION: u32 = 20;

#[derive(Serialize, Clone)]
pub enum ServoState {
    Locked,
    Unlocked
}
//
//impl ServoState {
//
//    fn toggle(&self) -> ServoState {
//        let cupi = CuPi::new().unwrap();
//        let mut pinout = cupi.pin(SERVO_GPIO_PIN)
//            .unwrap()
//            .high()
//            .output();
//
//        match *self {
//            ServoState::Locked => {
//
//                for _ in 0..DISTANCE_ITERATION {
//                    pinout.high().unwrap();
//                    delay_ms(ROTATE_CLOCKWISE_DELAY);
//                    pinout.low().unwrap();
//                    delay_ms(STANDARD_DELAY);
//                }
//
//                ServoState::Unlocked
//            }
//
//            ServoState::Unlocked => {
//                for _ in 0..DISTANCE_ITERATION {
//                    pinout.high().unwrap();
//                    delay_ms(ROTATE_COUNTERCLOCKWISE_DELAY);
//                    pinout.low().unwrap();
//                    delay_ms(STANDARD_DELAY);
//                }
//                ServoState::Locked
//            }
//        }
//    }
//}



impl ServoState {
    fn toggle(&self) -> ServoState {
        println!("Received a message.");
        match *self {
            ServoState::Locked => ServoState::Unlocked,
            ServoState::Unlocked => ServoState::Locked
        }
    }
}



#[post("/")]
fn toggle_servo_endpoint(servo: State<ServoState>) -> Json<ServoState> {
    let servo = servo.toggle(); // control the motor and toggle the state
    Json(servo.clone())
}


fn main() {
    let mut servo_position = ServoState::Locked;

    let pulse_pin = Pin::new(16); // Targeting pin 16 for now
    pulse_pin.with_exported(|| {
        sleep(Duration::from_milis(80)); // udev is apparently aweful
        loop {
            pulse_pin.set_value(0).expect("Couldn't set pin to low");
            sleep(Duration::from_millis(20)); // stay low for 20 ms
            pulse_pin.set_value(1).expect("Couldn't set pin to high");
            sleep(Duration::from_micros(1_500)); // go high for 1.5 ms
        }
    }).unwrap();

    rocket::ignite()
        .manage(pulse_pin)
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
