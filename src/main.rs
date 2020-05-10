use std::process::Command;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Trigger;

const BTN_PIN: u8 = 3;
const COUNT_MAX: u16 = 400;
const WAIT_MS: u64 = 10;

fn main() {
    // config gpio
    let gpio = Gpio::new().unwrap();
    let mut btn_pin = gpio.get(BTN_PIN).unwrap().into_input();

    // set interrupt
    btn_pin.set_interrupt(Trigger::FallingEdge).unwrap();

    loop {
        btn_pin.poll_interrupt(true, None).unwrap();

        let mut counter = 0;

        loop {
            if btn_pin.is_low() {
                counter += 1;
                if counter > COUNT_MAX {
                    // exec shutdown command
                    Command::new("shutdown")
                        .args(&["-h", "now"])
                        .output()
                        .expect("failed to execute process");
                    break;
                }
            } else {
                break;
            }

            thread::sleep(Duration::from_millis(WAIT_MS));
        }
    }
}
