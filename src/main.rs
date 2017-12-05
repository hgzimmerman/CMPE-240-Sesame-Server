#![feature(plugin)]
#![plugin(rocket_codegen)]
//#![feature(drop_types_in_const)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
//#![feature(const_fn)]



extern crate rocket;
extern crate rocket_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

//extern crate cupi;



use rocket_contrib::{Json, Value};
use rocket::State;
//use cupi::{CuPi, delay_ms, DigitalWrite};




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

    rocket::ignite()
        .manage(servo_position)
        .mount("/", routes![toggle_servo_endpoint])
        .launch();   
}
