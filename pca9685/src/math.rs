use thiserror::Error;

/// Represents the possible errors that can occur during computation.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents an error where the prescale value is out of bounds of u8.
    #[error("Prescale {0} out of bounds of u8")]
    PrescaleOutOfBounds(f64),

    /// Represents an error where the duty cycle is out of bounds of 0.0 to 1.0.
    #[error("Duty cycle {0} out of bounds of 0.0 to 1.0")]
    DutyCycleOutOfBounds(f64),
}

/// Computes the prescale value for a PCA9685 PWM controller based on the oscillator clock
///  frequency and desired update rate.
///
/// # Arguments
///
/// * `osc_clock` - The oscillator clock frequency in Hz.
/// * `update_rate` - The desired update rate in Hz.
///
/// # Returns
///
/// Returns a `Result` containing the prescale value as a `u8` if the computation is successful.
///  Otherwise, it returns an `Error` with a custom error message.
#[allow(unused)]
pub fn compute_prescale(osc_clock: u32, update_rate: u16) -> Result<u8, Error> {
    // Compute the prescale value using the formula: (osc_clock / (4096 * update_rate)) - 1.
    let prescale_value = (osc_clock as f64 / (4096_f64 * update_rate as f64)).round() - 1_f64;

    // Check if the prescale value is outside the bounds of u8.
    if prescale_value < u8::MIN as f64 || prescale_value > u8::MAX as f64 {
        // Return an error with a custom error message.
        return Err(Error::PrescaleOutOfBounds(prescale_value));
    }

    // Return the prescale value as a u8.
    Ok(prescale_value as u8)
}

/// Calculates the on time for a PWM signal based on the given duty cycle.
///
/// # Arguments
///
/// * `duty_cycle` - The duty cycle of the PWM signal, ranging from 0.0 to 1.0.
///
/// # Returns
///
/// The on time for the PWM signal, represented as a 16-bit unsigned integer.
#[allow(unused)]
pub fn compute_on_off_time(duty_cycle: f64) -> Result<(u16, u16), Error> {
    // Check if the duty cycle is outside the bounds of 0.0 to 1.0.
    if duty_cycle < 0.0 || duty_cycle > 1.0 {
        // Return an error with a custom error message.
        return Err(Error::DutyCycleOutOfBounds(duty_cycle));
    }

    // Calculate the on time based on the duty cycle.
    let on_time = (duty_cycle * 4095_f64).round() as u16;
    // Calculate the off time as the difference between the maximum value (4095) and the on time.
    let off_time = 4095_u16 - on_time;

    // Return the on time and off time as a tuple.
    Ok((on_time, off_time))
}
