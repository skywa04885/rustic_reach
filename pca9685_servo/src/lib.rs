use std::time::Duration;

use math::compute_duty_cycle;
use thiserror::Error;
use tokio::time::sleep;

pub(crate) mod math;

#[derive(Error, Debug)]
pub enum Error {
    #[error("PCA9685 Error: {0}")]
    PCA9685Error(#[from] pca9685::Error),
}

/// Represents the settings for a servo.
pub struct ServoSettings {
    /// The starting angle of the servo.
    pub(crate) start_angle: f64,
    /// The ending angle of the servo.
    pub(crate) end_angle: f64,
    /// The starting duty cycle of the servo.
    pub(crate) start_duty_cycle: f64,
    /// The ending duty cycle of the servo.
    pub(crate) end_duty_cycle: f64,
}

impl ServoSettings {
    pub const DEFAULT_START_ANGLE: f64 = 0_f64;
    pub const DEFAULT_END_ANGLE: f64 = 180_f64;
    pub const DEFAULT_START_DUTY_CYCLE: f64 = 0.05_f64;
    pub const DEFAULT_END_DUTY_CYCLE: f64 = 0.1_f64;

    /// Creates a new `Settings` instance with default values.
    ///
    /// # Returns
    ///
    /// The new `Settings` instance.
    pub fn new() -> Self {
        Self {
            start_angle: Self::DEFAULT_START_ANGLE,
            end_angle: Self::DEFAULT_END_ANGLE,
            start_duty_cycle: Self::DEFAULT_START_DUTY_CYCLE,
            end_duty_cycle: Self::DEFAULT_END_DUTY_CYCLE,
        }
    }

    /// Sets the starting angle of the servo and returns the modified `Settings` instance.
    ///
    /// # Arguments
    ///
    /// * `start_angle`: The starting angle of the servo.
    ///
    /// # Returns
    ///
    /// The modified `Settings` instance.
    pub fn with_start_angle(mut self, start_angle: f64) -> Self {
        self.start_angle = start_angle;
        self
    }

    /// Sets the ending angle of the servo and returns the modified `Settings` instance.
    ///
    /// # Arguments
    ///
    /// * `end_angle`: The ending angle of the servo.
    ///
    /// # Returns
    ///
    /// The modified `Settings` instance.
    pub fn with_end_angle(mut self, end_angle: f64) -> Self {
        self.end_angle = end_angle;
        self
    }

    /// Sets the starting duty cycle of the servo and returns the modified `Settings` instance.
    ///
    /// # Arguments
    ///
    /// * `start_duty_cycle`: The starting duty cycle of the servo.
    ///
    /// # Returns
    ///
    /// The modified `Settings` instance.
    pub fn with_start_duty_cycle(mut self, start_duty_cycle: f64) -> Self {
        self.start_duty_cycle = start_duty_cycle;
        self
    }

    /// Sets the ending duty cycle of the servo and returns the modified `Settings` instance.
    ///
    /// # Arguments
    ///
    /// * `end_duty_cycle`: The ending duty cycle of the servo.
    ///
    /// # Returns
    ///
    /// The modified `Settings` instance.
    pub fn with_end_duty_cycle(mut self, end_duty_cycle: f64) -> Self {
        self.end_duty_cycle = end_duty_cycle;
        self
    }
}

/// Represents a servo motor.
pub struct Servo {
    /// The channel of the PCA9685 controller that the servo is connected to.
    channel: pca9685::Channel,
    /// The settings for the servo.
    settings: ServoSettings,
    /// The current angle of the servo.
    angle: f64,
}

impl Servo {
    /// Creates a new Servo instance.
    ///
    /// # Arguments
    ///
    /// * `channel` - The PCA9685 channel to which the servo is connected.
    /// * `settings` - The settings for the servo.
    /// * `angle` - The initial angle of the servo.
    ///
    /// # Returns
    ///
    /// Returns a new Servo instance.
    pub fn new(channel: pca9685::Channel, settings: ServoSettings, angle: f64) -> Self {
        Self {
            channel,
            settings,
            angle,
        }
    }

    /// Writes the servo to a desired angle with a specified speed.
    ///
    /// This method calculates the necessary steps to reach the desired angle
    /// based on the servo's settings and the given speed. It then iterates
    /// through the steps, gradually updating the servo's angle until it reaches
    /// the desired angle. The speed parameter determines the interval between
    /// sequential updates.
    ///
    /// # Arguments
    ///
    /// * `angle` - The desired angle to set the servo to.
    /// * `speed` - The speed at which the servo should move to the desired angle.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the servo is successfully written to the desired angle,
    /// otherwise returns an `Error` indicating the failure.
    pub async fn write_with_speed(&mut self, angle: f64, speed: f64) -> Result<(), Error> {
        // Destructure the settings for easier access.
        let ServoSettings {
            start_duty_cycle,
            end_duty_cycle,
            start_angle,
            end_angle,
        } = self.settings;

        // Calculate the range of the servo's angle (the range of the servo in radians).
        let servo_angle_range = end_angle - start_angle;

        // Calculate the steps within the duty cycle.
        let steps_within_duty_cycle = (end_duty_cycle - start_duty_cycle) * 4095.0;

        // Calculate the step size (this gives the size of a single step in radians).
        let step_size = servo_angle_range / steps_within_duty_cycle;

        // Compute the update frequency, and use it to compute the interval between sequential updates.
        let frequency = speed / step_size;
        let interval = 1.0 / frequency;

        // Convert the interval into a duration for sleeping.
        let sleep_duration = Duration::from_secs_f64(interval);

        // Compute the number of iterations we need to perform to reach the desired angle.
        let iterations = ((angle - self.angle).abs() / step_size).round() as usize;

        // Store the initial angle.
        let start_angle = self.angle;

        // Perform the iterations to reach the desired angle.
        for i in 0..iterations {
            let current_angle = i as f64 * step_size + start_angle;
            self.write(current_angle).await?;
            sleep(sleep_duration).await;
        }

        // Return success.
        Ok(())
    }

    /// Writes the servo to a desired angle.
    ///
    /// This method calculates the duty cycle based on the servo's settings and
    /// the desired angle. It then writes the duty cycle to the servo's channel.
    ///
    /// # Arguments
    ///
    /// * `angle` - The desired angle to set the servo to.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the servo is successfully written to the desired angle,
    /// otherwise returns an `Error` indicating the failure.
    pub async fn write(&mut self, angle: f64) -> Result<(), Error> {
        // Get the required parameters from the settings.
        let ServoSettings {
            start_duty_cycle,
            end_duty_cycle,
            start_angle,
            end_angle,
        } = self.settings;

        // Compute the duty cycle to write based on the settings and the desired angle.
        let duty_cycle = compute_duty_cycle(
            start_duty_cycle,
            end_duty_cycle,
            start_angle,
            end_angle,
            angle,
        );

        // Write the duty cycle to the channel.
        self.channel.write_duty_cycle(duty_cycle).await?;

        // Return success.
        Ok(())
    }
}
