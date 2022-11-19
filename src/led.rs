//! On board LED
//! - Red 1: GPIO12/SPIHOLD
//! - Red 2: GPIO13/SPIWP
use embedded_hal::digital::blocking::{OutputPin, StatefulOutputPin};
use esp_idf_hal::gpio::Output;
use esp_idf_hal::gpio::{Gpio12, Gpio13};

/// Red LED 1
pub struct Led1 {
    port: Gpio12<Output>,
}

impl Led1 {
    /// Use an output-capable pin (here fixed gpio12)
    pub fn new<T: Send>(port: Gpio12<T>) -> Self {
        Self {
            port: port.into_output().unwrap(),
        }
    }
}

/// Red LED 2
pub struct Led2 {
    port: Gpio13<Output>,
}

impl Led2 {
    /// Use an output-capable pin (here fixed gpio13)
    pub fn new<T: Send>(port: Gpio13<T>) -> Self {
        Self {
            port: port.into_output().unwrap(),
        }
    }
}

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);

    /// Check the LED status
    fn is_on(&mut self) -> bool;

    /// Toggle the LED status
    fn toggle(&mut self) {
        match self.is_on() {
            true => self.off(),
            false => self.on(),
        }
    }
}

impl Led for Led1 {
    fn off(&mut self) {
        self.port.set_high().unwrap();
    }

    fn on(&mut self) {
        self.port.set_low().unwrap();
    }

    fn is_on(&mut self) -> bool {
        self.port.is_set_low().unwrap()
    }
}

impl Led for Led2 {
    fn off(&mut self) {
        self.port.set_high().unwrap();
    }

    fn on(&mut self) {
        self.port.set_low().unwrap();
    }

    fn is_on(&mut self) -> bool {
        self.port.is_set_low().unwrap()
    }
}
