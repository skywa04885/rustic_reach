use rppal::i2c::I2c;
use thiserror::Error;
use tokio::time::sleep;

/// Represents the possible errors that can occur during I2C communication with the PCA9685 device.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents an I2C error.
    #[error("I2C Error: {0}")]
    I2CError(#[from] rppal::i2c::Error),
}

/// Represents a PCA9685 device.
pub struct Device {
    i2c: I2c,
    address: u16,
}

impl Device {
    /// Creates a new `Device` instance with the specified I2C context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The I2C context to use for communication with the device.
    /// * `address` - The I2C address of the device.
    ///
    /// # Returns
    ///
    /// Returns a new `Device` instance.
    #[allow(unused)]
    pub fn new(i2c: I2c, address: u16) -> Self {
        Self { i2c, address }
    }

    /// Reads a single byte from the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to read from.
    ///
    /// # Returns
    ///
    /// Returns the read byte on success, or an `Error` if the read operation fails.
    #[allow(unused)]
    pub(crate) fn read_byte(&mut self, address: u8) -> Result<u8, Error> {
        let write_buffer = [address];
        let mut read_buffer = [0_u8];

        self.i2c.write_read(&write_buffer, &mut read_buffer)?;

        Ok(read_buffer[0])
    }

    /// Reads multiple bytes from the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to read from.
    /// * `read_buffer` - A mutable reference to a buffer where the read bytes will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the read operation fails.
    #[allow(unused)]
    pub(crate) fn read_bytes(&mut self, address: u8, read_buffer: &mut [u8]) -> Result<(), Error> {
        let write_buffer = [address];

        self.i2c.write_read(&write_buffer, read_buffer)?;

        Ok(())
    }

    /// Writes a single byte to the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to write to.
    /// * `value` - The byte value to write.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the write operation fails.
    #[allow(unused)]
    pub(crate) fn write_byte(&mut self, address: u8, value: u8) -> Result<(), Error> {
        let buffer = &[address, value];

        self.i2c.write(buffer)?;

        Ok(())
    }

    /// Performs a software reset on the PCA9685 device.
    ///
    /// This function sends a reset command to the device, which resets all internal registers and
    /// settings to their default values. It does not affect the I2C communication settings.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the reset operation fails.
    pub async fn software_reset(&mut self) -> Result<(), Error> {
        // Create the buffer that will contain the reset command (as described in section "7.6").
        let buffer = [0x06];

        // Set the slave address to the general call address (as described in section "7.6").
        self.i2c.set_slave_address(0x00)?;

        // Write the reset command to the device.
        self.i2c.write(&buffer)?;

        // Set the slave address back to the device address.
        self.i2c.set_slave_address(self.address)?;

        // Wait for the reset to complete.
        sleep(std::time::Duration::from_millis(1)).await;

        // Return success.
        Ok(())
    }

    /// Writes multiple bytes to the device at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to write to.
    /// * `buffer` - A slice containing the bytes to write.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the write operation fails.
    #[allow(unused)]
    pub(crate) fn write_bytes(&mut self, address: u8, buffer: &[u8]) -> Result<(), Error> {
        let mut write_buffer = Vec::with_capacity(buffer.len() + 1);

        write_buffer.push(address);
        write_buffer.extend_from_slice(buffer);

        self.i2c.write(&write_buffer)?;

        Ok(())
    }

    /// Sets the specified bits in a byte at the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to write to.
    /// * `mask` - The bits to set in the byte.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the write operation fails.
    #[allow(unused)]
    pub(crate) fn set_bit_mask(&mut self, address: u8, mask: u8) -> Result<(), Error> {
        let mut value = self.read_byte(address)?;

        value |= mask;

        self.write_byte(address, value)?;

        Ok(())
    }

    /// Clears the specified bits in a byte at the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the device to write to.
    /// * `mask` - The bits to clear in the byte.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the write operation fails.
    #[allow(unused)]
    pub(crate) fn clear_bit_mask(&mut self, address: u8, mask: u8) -> Result<(), Error> {
        let mut value = self.read_byte(address)?;

        value &= !mask;

        self.write_byte(address, value)?;

        Ok(())
    }
}
