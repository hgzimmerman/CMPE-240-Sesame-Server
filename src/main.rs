#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(const_fn)]
#![feature(duration_from_micros)]

// Bring in external crates (libraries).
// Only the main.rs file is responsible for bringing crates into scope.
extern crate sysfs_gpio;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rocket;
extern crate rocket_contrib;

// Import elements from the crates.
use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::sync::Mutex;
use std::fs::File;
use rocket::State;

// Use the servo file
mod servo;
use servo::Servo;

/// API endpoint that Rocket will route POST requests with empty bodies to.
/// This will toggle the servo's position.
#[post("/")]
fn toggle_servo_endpoint(servo: State<Mutex<Servo>>) {
    info!("Got message");
    // Lock the mutex and toggle the servo's state
    servo.lock().unwrap().toggle();
    // The lock will be dropped at the end of this function by a RAII destructor.
}

/// The pin used to drive the signal to the physical servo motor.
const SERVO_PIN_NUMBER: u64 = 16;

/// Main function.
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

    // Start the server
    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
