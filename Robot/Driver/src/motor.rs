use gpio::{GpioIn, GpioOut};
use std::str;

pub struct Motor {
    pub number: u8,
    pub pin: gpio::sysfs::SysFsGpioOutput,
    pub fd: i32
}

impl Motor {
    pub fn new(pin_number: u16, motor_number: u8) -> Self {
        return Motor {
            number: motor_number,
            pin: gpio::sysfs::SysFsGpioOutput::open(pin_number).unwrap(),
            fd: 0
        }
    }

    pub fn select(mut self) {
        self.pin.set_value(true).expect(&(String::from("Could not select motor #") + &String::from(str::from_utf8(&[self.number]).unwrap())))
    }
}