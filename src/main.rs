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



/// Holds information on the current state of the servo.
#[derive(Clone)]
pub enum ServoState {
    Locked,
    Unlocked
}

/// Wrapper around the Servo's state and the pin used to send the signal to the servo.
pub struct Servo{
    state: ServoState,
    signal_pin: Pin
}



impl Servo {
    fn toggle(&mut self) {
        println!("Toggling servo state:");
        // Set the state to the new servo state.
        self.state = match self.state {
            ServoState::Locked => {
                println!("Unlocking");
                self.unlock();
                ServoState::Unlocked
            },
            ServoState::Unlocked => {
                println!("Locking");
                self.lock();
                ServoState::Locked
            }
        }
    }

    /// Move the servo into the "lock" position.
    fn lock(&self) {
        send_pulses(self.signal_pin.clone(), Duration::from_micros(LOCK_PULSE_WIDTH_MICROS));
    }
    /// Move the servo into the "unlock" position.
    fn unlock(&self) {
        send_pulses(self.signal_pin.clone(), Duration::from_micros(UNLOCK_PULSE_WIDTH_MICROS));
    }
}

const UNLOCK_PULSE_WIDTH_MICROS: u64 = 2000; // Stay high for 2 ms
const LOCK_PULSE_WIDTH_MICROS: u64 = 1000; // Stay high for 1 ms





/// The Servo expects a signal every 20 ms.
/// The signal shall go high for the pulse_width parameter.
/// Depending on how long the width is (usually between 1-2 ms), the servo will rotate to a given angle.
///
/// Once signals stop, the servo will remain in its last position.
fn send_pulses(pulse_pin: Pin, pulse_width: Duration) {
    pulse_pin.with_exported(|| {
        sleep(Duration::from_millis(180)); // udev is apparently aweful, and takes a while to set the permissions of the pin.
        pulse_pin.set_direction(Direction::Low).expect("Couldn't set the direction of the pin");
        // loop for about a tenth of a second
        for _ in 0..50 {
            pulse_pin.set_value(0).expect("Couldn't set pin to low");
            sleep(Duration::from_millis(20) - pulse_width); // stay low for 20 ms - the width of the pulse
            pulse_pin.set_value(1).expect("Couldn't set pin to high");
            sleep(pulse_width); // stay high for the provided pulse width
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
    let servo_position = Mutex::new(
        Servo {
            state: ServoState::Locked,
            signal_pin: Pin::new(16)
        }
    );

    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
