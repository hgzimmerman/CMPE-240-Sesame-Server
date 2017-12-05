use sysfs_gpio::{Direction, Pin};
use std::time::Duration;
use std::thread::sleep;

/// Holds information on the current state of the servo.
#[derive(Clone, Debug)]
enum ServoState {
    Locked,
    Unlocked
}

/// Wrapper around the Servo's state and the pin used to send the signal to the servo.
#[derive(Clone, Debug)]
pub struct Servo {
    state: ServoState,
    signal_pin: Pin
}


impl Servo {

    /// Constructs a new Servo instance with a given pin number used to drive the
    /// signal to the physical servo.
    pub fn new(pin_number: u64) -> Servo {
        Servo {
            state: ServoState::Locked,
            signal_pin: Pin::new(pin_number)
        }
    }

    /// Depending on the current rotational state of the servo, move the servo into the other state
    pub fn toggle(&mut self) {
        info!("Toggling servo state:");
        // Set the state to the new servo state.
        self.state = match self.state {
            ServoState::Locked => {
                self.unlock();
                ServoState::Unlocked
            },
            ServoState::Unlocked => {
                self.lock();
                ServoState::Locked
            }
        }
    }

    /// Move the servo into the "locked" position.
    fn lock(&self) {
        info!("Locking");
        self.send_pulses(Duration::from_micros(LOCK_PULSE_WIDTH_MICROS));
        info!("Servo now in locked position");
    }

    /// Move the servo into the "unlocked" position.
    fn unlock(&self) {
        info!("Unlocking");
        self.send_pulses( Duration::from_micros(UNLOCK_PULSE_WIDTH_MICROS));
        info!("Servo now in unlocked position");
    }


    /// The Servo expects a signal every 20 ms.
    /// The signal shall go high for the duration of pulse_width parameter.
    /// Depending on how long the width is (usually between 1-2 ms), the servo will rotate to a given angle.
    ///
    /// Once signals stop, the servo will remain in its last position.
    fn send_pulses(&self, pulse_width: Duration) {
        let pulse_pin = self.signal_pin.clone();
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

}

const UNLOCK_PULSE_WIDTH_MICROS: u64 = 2000; // Stay high for 2 ms
const LOCK_PULSE_WIDTH_MICROS: u64 = 1000; // Stay high for 1 ms