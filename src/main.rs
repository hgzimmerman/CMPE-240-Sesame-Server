#![feature(plugin)]
#![plugin(rocket_codegen)]
//#![feature(drop_types_in_const)]
#![feature(const_fn)]
//#![feature(const_fn)]
#![feature(duration_from_micros)]


extern crate sysfs_gpio;


#[macro_use]
extern crate log;
extern crate simplelog;

extern crate rocket;
extern crate rocket_contrib;


use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::sync::Mutex;
use std::fs::File;
use rocket::State;

mod servo;
use servo::Servo;



#[post("/")]
fn POST_toggle_servo_endpoint(servo: State<Mutex<Servo>>) {
    info!("Got message");
    // Lock the mutex and toggle the servo's state
    servo.lock().unwrap().toggle();
    // The lock will be dropped at the end of this function by a RAII destructor.
}

#[get("/")]
fn GET_toggle_servo_endpoint(servo: State<Mutex<Servo>>) {
    info!("Got message");
    // Lock the mutex and toggle the servo's state
    servo.lock().unwrap().toggle();
    // The lock will be dropped at the end of this function by a RAII destructor.
}

/// The pin used to drive the signal to the physical servo motor.
const SERVO_PIN_NUMBER: u64 = 16;

fn main() {
    // Initialize the Servo, and protect it from synchronous access with a Mutex.
    let servo_position = Mutex::new( Servo::new(SERVO_PIN_NUMBER) );

    // Set up logging
    const LOGFILE_NAME: &'static str = "servo.log";
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
        ]
    ).unwrap();

    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![POST_toggle_servo_endpoint, GET_toggle_servo_endpoint])
        .launch();   
}
