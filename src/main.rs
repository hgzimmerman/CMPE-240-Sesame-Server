#![feature(plugin)]
#![plugin(rocket_codegen)]
//#![feature(drop_types_in_const)]
#![feature(const_fn)]
//#![feature(const_fn)]
#![feature(duration_from_micros)]


extern crate sysfs_gpio;



extern crate rocket;
extern crate rocket_contrib;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::sync::Mutex;

//use rocket_contrib::{Json, Value};
use rocket::State;



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

const UNLOCK_PULSE_WIDTH_MICROS: u64 = 2000; // Stay high for 2 ms
const LOCK_PULSE_WIDTH_MICROS: u64 = 1000; // Stay high for 1 ms

fn unlock(pulse_pin: Pin) {
    send_pulse(pulse_pin, Duration::from_micros(UNLOCK_PULSE_WIDTH_MICROS));
}

fn lock(pulse_pin: Pin) {
    send_pulse(pulse_pin, Duration::from_micros(LOCK_PULSE_WIDTH_MICROS));

}

fn send_pulse(pulse_pin: Pin, pulse_width: Duration) {
    pulse_pin.with_exported(|| {
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        pulse_pin.set_direction(Direction::Low).expect("Couldn't set the direction of the pin");
        // loop for about a tenth of a second
        for _ in 0..50 {
            pulse_pin.set_value(0).expect("Couldn't set pin to low");
            sleep(Duration::from_millis(20) - pulse_width); // stay low for 20 ms - the width of the pulse
            pulse_pin.set_value(1).expect("Couldn't set pin to high");
            sleep(pulse_width); // stay high for the given time
        }

        Ok(())
    }).unwrap();
}


#[post("/")]
fn toggle_servo_endpoint(servo: State<Mutex<Servo>>) {
    println!("Got message");
    servo.lock().unwrap().toggle();
}


fn main() {
    let servo_position = Mutex::new(Servo(ServoState::Locked(Pin::new(16)) ));

    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
