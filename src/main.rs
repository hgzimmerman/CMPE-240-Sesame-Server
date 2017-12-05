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
use std::sync::Mutex;


use rocket_contrib::{Json, Value};
use rocket::State;



const SERVO_GPIO_PIN: u32 = 18;

const ROTATE_CLOCKWISE_DELAY:u32 = 1;
const ROTATE_COUNTERCLOCKWISE_DELAY: u32 = 2;
const STANDARD_DELAY: u32 = 1;
const DISTANCE_ITERATION: u32 = 20;

#[derive(Clone)]
pub enum ServoState {
    Locked(Pin),
    Unlocked(Pin)
}

pub struct Servo(ServoState);



impl Servo {
    fn toggle(&mut self) {
        println!("Toggling servo state:");
        // Set the state to the new servo state.
        self.0 = match self.0 {
            ServoState::Locked(pin) => {
                println!("Unlocking");
                unlock(pin.clone());
                ServoState::Unlocked(pin)
            },
            ServoState::Unlocked(pin) => {
                println!("Locking");
                lock(pin.clone());
                ServoState::Locked(pin)
            }
        }
    }
}

fn unlock(pulse_pin: Pin) {
//    let pulse_pin = Pin::new(16); // Targeting pin 16 for now
    pulse_pin.with_exported(|| {
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        pulse_pin.set_direction(Direction::Low).expect("Couldn't set the direction of the pin");
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        for _ in 0..500 {
            pulse_pin.set_value(0).expect("Couldn't set pin to low");
            sleep(Duration::from_millis(20)); // stay low for 20 ms
            pulse_pin.set_value(1).expect("Couldn't set pin to high");
            sleep(Duration::from_micros(2_000)); // go high for 2 ms
        }
        Ok(())
    }).unwrap();
}

fn lock(pulse_pin: Pin) {
    pulse_pin.with_exported(|| {
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        pulse_pin.set_direction(Direction::Low).expect("Couldn't set the direction of the pin");
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        // loop for about a second
        for _ in 0..500 {
            pulse_pin.set_value(0).expect("Couldn't set pin to low");
            sleep(Duration::from_millis(20)); // stay low for 20 ms
            pulse_pin.set_value(1).expect("Couldn't set pin to high");
            sleep(Duration::from_micros(1_000)); // go high for 1 ms
        }

        Ok(())
    }).unwrap();
}



#[post("/")]
fn toggle_servo_endpoint(servo: State<Mutex<Servo>>) {
//    let servo = servo.toggle(); // control the motor and toggle the state

    println!("Got message");
    servo.inner().lock().unwrap().toggle();
//    let pulse_pin = Pin::new(16); // Targeting pin 16 for now
//    pulse_pin.with_exported(|| {
//        pulse_pin.set_direction(Direction::Low).expect("Couldn't set the direction of the pin");
//        sleep(Duration::from_millis(80)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
//        // loop for about a second
//        for _ in 0..200 {
//            pulse_pin.set_value(0).expect("Couldn't set pin to low");
//            sleep(Duration::from_millis(20)); // stay low for 20 ms
//            pulse_pin.set_value(1).expect("Couldn't set pin to high");
//            sleep(Duration::from_micros(2_000)); // go high for 1.5 ms
//        }
//
//        for _ in 0..50 {
//            pulse_pin.set_value(0).expect("Couldn't set pin to low");
//            sleep(Duration::from_millis(20)); // stay low for 20 ms
//            pulse_pin.set_value(1).expect("Couldn't set pin to high");
//            sleep(Duration::from_micros(1_000)); // go high for 1 ms
//        }
//        Ok(())
//    }).unwrap();

    println!("done doing servo stuff")

//    Json(servo.clone())
}


fn main() {
    let mut servo_position = Mutex::new(Servo(ServoState::Locked(Pin::new(16)) ));

    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
