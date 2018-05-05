use embedded_hal::digital::OutputPin;

use command::Command;
use registers::{Register, Config, Status};

/// Trait that hides all the GPIO/SPI type parameters for use by the
/// operation modes
pub trait Device {
    type Error;

    fn delay_us(&mut self, delay: u16);

    fn ce_enable(&mut self);
    fn ce_disable(&mut self);
    /// Helper; the receiving during RX and sending during TX require `CE`
    /// to be low.
    fn with_ce_disabled<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.ce_disable();
        let r = f(self);
        self.ce_enable();
        r
    }

    fn send_command<C: Command>(&mut self, command: &C) -> Result<(Status, C::Response), Self::Error>;
    fn write_register<R: Register>(&mut self, register: R) -> Result<Status, Self::Error>;
    fn read_register<R: Register>(&mut self) -> Result<(Status, R), Self::Error>;

    fn update_config<F, R>(&mut self, f: F) -> Result<R, Self::Error>
        where F: FnOnce(&mut Config) -> R;
}