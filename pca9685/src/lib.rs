use std::{sync::Arc, time::Duration};

use device::Device;
use math::{compute_on_off_time, compute_prescale};
use memory::{
    led_on_l_addr, MODE1_ADDR, MODE1_ALLCALL_BIT, MODE1_RESTART_BIT, MODE1_SLEEP_BIT,
    PRE_SCALE_ADDR,
};
use rppal::gpio::{Level, OutputPin};
use thiserror::Error;
use tokio::{sync::Mutex, time::sleep};

pub mod device;
pub(crate) mod math;
pub(crate) mod memory;

/// Represents the possible errors that can occur in the PCA9685 driver.
#[derive(Debug, Error)]
pub enum Error {
    /// Device error: an error occurred while communicating with the PCA9685 device.
    #[error("Device error: {0}")]
    DeviceError(#[from] device::Error),
    /// Math error: an error occurred during mathematical calculations.
    #[error("Math error: {0}")]
    MathError(#[from] math::Error),
    /// Restart error: an error occurred during the restart operation of the PCA9685 device.
    #[error("Restart error")]
    RestartError,
}

/// Builder for creating a `Driver` instance with custom configuration.
pub struct DriverBuilder {
    device: Device,
    oe: OutputPin,
    osc_clock: u32,
    update_rate: u16,
}

impl DriverBuilder {
    /// Creates a new instance of the `DriverBuilder` struct with default values for the oscillator clock and update rate.
    ///
    /// # Arguments
    ///
    /// * `device` - The `Device` instance used for communication with the PCA9685 device.
    /// * `oe` - The `OutputPin` instance used for controlling the Output Enable pin of the PCA9685 device.
    ///
    /// # Returns
    ///
    /// A new instance of the `DriverBuilder` struct with default values for the oscillator clock (50,000,000) and update rate (50).
    pub fn new(device: Device, oe: OutputPin) -> Self {
        Self {
            device,
            oe,
            osc_clock: 50_000_000_u32,
            update_rate: 50_u16,
        }
    }

    /// Sets the oscillator clock value for the `DriverBuilder`.
    ///
    /// This function allows you to customize the oscillator clock value used by the `DriverBuilder`.
    ///
    /// # Arguments
    ///
    /// * `osc_clock` - The oscillator clock value to set.
    ///
    /// # Returns
    ///
    /// Returns the modified `DriverBuilder` instance.
    pub fn with_osc_clock(mut self, osc_clock: u32) -> Self {
        self.osc_clock = osc_clock;
        self
    }

    /// Sets the update rate value for the `DriverBuilder`.
    ///
    /// This function allows you to customize the update rate value used by the `DriverBuilder`.
    ///
    /// # Arguments
    ///
    /// * `update_rate` - The update rate value to set.
    ///
    /// # Returns
    ///
    /// Returns the modified `DriverBuilder` instance.
    pub fn with_update_rate(mut self, update_rate: u16) -> Self {
        self.update_rate = update_rate;
        self
    }

    /// Builds the `Driver` instance.
    ///
    /// This function finalizes the configuration of the `DriverBuilder` and creates a new
    /// instance of the `Driver` struct. It performs the following steps:
    /// 1. Clears the "LED All Calls" bit in the MODE1 register.
    /// 2. Computes the prescale value based on the oscillator clock and update rate.
    /// 3. Writes the prescale value to the PRE_SCALE register.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Driver` instance if the build operation is successful,
    /// otherwise returns an `Error`.
    pub fn build(mut self) -> Result<Driver, Error> {
        // Write high to oe.
        self.oe.write(Level::High);

        // Do not listen to "LED All Calls".
        self.device.clear_bit_mask(MODE1_ADDR, MODE1_ALLCALL_BIT)?;

        // Compute the prescale value.
        let prescale: u8 = compute_prescale(self.osc_clock, self.update_rate)?;

        // Write the prescale value to the device.
        self.device.write_byte(PRE_SCALE_ADDR, prescale)?;

        // Return the driver instance.
        Ok(Driver::new(self.device))
    }
}
/// Represents a driver for the PCA9685 device.
pub struct Driver {
    device: Device,
}

impl Driver {
    /// Creates a new instance of the `Driver` struct.
    ///
    /// # Arguments
    ///
    /// * `device` - The I2c context used for communication with the PCA9685 device.
    ///
    /// # Returns
    ///
    /// A new instance of the `Driver` struct.
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    /// Creates a new instance of the `DriverBuilder` struct.
    ///
    /// # Arguments
    ///
    /// * `device` - The `Device` instance used for communication with the PCA9685 device.
    /// * `oe` - The `OutputPin` instance used for controlling the Output Enable pin of the PCA9685 device.
    ///
    /// # Returns
    ///
    /// A new instance of the `DriverBuilder` struct.
    pub fn builder(device: Device, oe: OutputPin) -> DriverBuilder {
        DriverBuilder::new(device, oe)
    }

    /// Puts the PCA9685 device into sleep mode.
    ///
    /// This function sets the sleep bit in the MODE1 register to put the device into sleep mode.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the sleep operation is successful, otherwise returns an `Error`.
    pub fn sleep(&mut self) -> Result<(), Error> {
        self.device.set_bit_mask(MODE1_ADDR, MODE1_SLEEP_BIT)?;

        Ok(())
    }

    /// Wakes up the PCA9685 device from sleep mode.
    ///
    /// This function clears the sleep bit in the MODE1 register to awaken the device.
    /// It then waits for a short duration for the oscillator to settle.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the wake operation is successful, otherwise returns an `Error`.
    pub async fn wake(&mut self) -> Result<(), Error> {
        self.device.clear_bit_mask(MODE1_ADDR, MODE1_SLEEP_BIT)?;

        sleep(Duration::from_micros(500_u64)).await;

        Ok(())
    }

    /// Restarts the PCA9685 device.
    ///
    /// This function checks if the restart bit is set. If not, it waits for a short duration
    /// for the restart bit to be set. If the restart bit is still not set, it returns an error.
    /// If the restart bit is set, it clears the sleep bit to awaken the device and waits for
    /// the oscillator to settle. Then, it writes a logic 1 to the restart bit to clear it and
    /// restart all the channels.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the restart operation is successful, otherwise returns an `Error`.
    pub async fn restart(&mut self) -> Result<(), Error> {
        // If the restart bit is still zero, give some time for the restart bit to be set.
        //  this is talked about in the "remark" in section "7.3.1.1".
        if self.device.read_byte(MODE1_ADDR)? & MODE1_RESTART_BIT == 0 {
            sleep(Duration::from_micros(500_u64)).await;
        }

        // If the restart bit still is not set, return an error, because we cannot restart
        //  the channels.
        if self.device.read_byte(MODE1_ADDR)? & MODE1_RESTART_BIT == 0 {
            return Err(Error::RestartError);
        }

        // Clear the sleep bit to awaken the device, give some time for the oscilator to
        //  settle as specified in step 2 of the restart sequence in section "7.3.1.1".
        self.device.clear_bit_mask(MODE1_ADDR, MODE1_SLEEP_BIT)?;
        sleep(Duration::from_micros(500_u64)).await;

        // Write a logic 1 to the restart bit to clear it and restart all the channels,
        //  as specified in step 3 of the restart sequence in section "7.3.1.1.".
        self.device.set_bit_mask(MODE1_ADDR, MODE1_RESTART_BIT)?;

        // Return success.
        Ok(())
    }

    /// Writes the on and off values to the specified channel of the PCA9685 device.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel number to write the values to.
    /// * `on` - The on value to write.
    /// * `off` - The off value to write.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the write operation is successful, otherwise returns an `Error`.
    pub fn write_channel(&mut self, channel: u8, on: u16, off: u16) -> Result<(), Error> {
        assert!(on <= 4095_u16);
        assert!(off <= 4095_u16);

        // Get the base address of the registers for the given channel.
        let address: u8 = led_on_l_addr(channel);

        // Split the on value into two bytes.
        let on_l_val: u8 = ((on & 0x00FF_u16) >> 0_u16) as u8;
        let on_h_val: u8 = ((on & 0xFF00_u16) >> 8_u16) as u8;

        // Split the off value into two bytes.
        let off_l_val: u8 = ((off & 0x00FF_u16) >> 0_u16) as u8;
        let off_h_val: u8 = ((off & 0xFF00_u16) >> 8_u16) as u8;

        // Create a buffer with the values to write.
        let buffer: [u8; 4] = [on_l_val, on_h_val, off_l_val, off_h_val];

        // Write the values to the registers.
        self.device.write_bytes(address, &buffer)?;

        // Return success.
        Ok(())
    }

    /// Writes the duty cycle to the specified channel of the PCA9685 device.
    ///
    /// This function takes a channel number and a duty cycle as arguments and computes the
    /// corresponding on and off values based on the duty cycle. It then writes the on and off
    /// values to the register of the specified channel using the `write_channel` method.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel number to write the duty cycle to.
    /// * `duty_cycle` - The duty cycle to write, represented as a floating-point value between 0.0 and 1.0.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the write operation is successful, otherwise returns an `Error`.
    pub fn write_channel_duty_cycle(&mut self, channel: u8, duty_cycle: f64) -> Result<(), Error> {
        // Compute the on and off values based on the duty cycle.
        let (on, off) = compute_on_off_time(duty_cycle)?;

        // Write the on and off values to the register.
        self.write_channel(channel, on, off)?;

        // Return success.
        Ok(())
    }
}

/// Represents a channel of a PCA9685 driver.
pub struct Channel {
    driver: Arc<Mutex<Driver>>,
    channel: u8,
}

impl Channel {
    /// Creates a new `Channel` instance.
    ///
    /// # Arguments
    ///
    /// * `driver` - The driver for the PCA9685.
    /// * `channel` - The channel number.
    ///
    /// # Returns
    ///
    /// A new `Channel` instance.
    pub fn new(driver: Arc<Mutex<Driver>>, channel: u8) -> Self {
        Self { driver, channel }
    }

    /// Writes the on and off values to the channel.
    ///
    /// # Arguments
    ///
    /// * `on` - The on value.
    /// * `off` - The off value.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the write operation is successful, otherwise an `Err` containing the error.
    pub async fn write(&mut self, on: u16, off: u16) -> Result<(), Error> {
        self.driver
            .lock()
            .await
            .write_channel(self.channel, on, off)
    }

    /// Writes the duty cycle to the channel.
    ///
    /// # Arguments
    ///
    /// * `duty_cycle` - The duty cycle value.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the write operation is successful, otherwise an `Err` containing the error.
    pub async fn write_duty_cycle(&mut self, duty_cycle: f64) -> Result<(), Error> {
        self.driver
            .lock()
            .await
            .write_channel_duty_cycle(self.channel, duty_cycle)
    }
}
