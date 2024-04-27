use std::time::Duration;

use thiserror::Error;
use tokio::time::sleep;

use crate::{math::compute_duty_cycle, settings::ServoSettings};

#[derive(Error, Debug)]
pub enum Error {
    #[error("PCA9685 Error: {0}")]
    PCA9685Error(#[from] pca9685::Error),
}

pub struct ServoWriter {
    channel: pca9685::Channel,
    settings: ServoSettings,
    angle_sender: tokio::sync::watch::Sender<f64>,
}

impl ServoWriter {
    pub(crate) fn new(
        channel: pca9685::Channel,
        settings: ServoSettings,
        angle_sender: tokio::sync::watch::Sender<f64>,
    ) -> Self {
        Self {
            channel,
            settings,
            angle_sender,
        }
    }

    pub async fn write_with_duration(
        &mut self,
        target_angle: f64,
        duration: f64,
    ) -> Result<(), Error> {
        // Get the current angle.
        let current_angle = *self.angle_sender.borrow();

        // Compute the speed to make the movement last the gicen duration.
        let speed = (target_angle - current_angle).abs() / duration;

        // Write the angle with the computed speed.
        self.write_with_speed(target_angle, speed).await?;

        // Return success.
        Ok(())
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
    /// * `target_angle` - The desired angle to set the servo to.
    /// * `speed` - The speed at which the servo should move to the desired angle.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the servo is successfully written to the desired angle,
    /// otherwise returns an `Error` indicating the failure.
    pub async fn write_with_speed(&mut self, target_angle: f64, speed: f64) -> Result<(), Error> {
        // Destructure the settings for easier access.
        let ServoSettings {
            start_duty_cycle,
            end_duty_cycle,
            start_angle,
            end_angle,
        } = self.settings;

        // Get the current angle.
        let mut current_angle = *self.angle_sender.borrow();

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
        let iterations = ((target_angle - current_angle).abs() / step_size).round() as usize;

        // Perform the iterations to reach the desired angle.
        for i in 0..iterations {
            // Compute the new current angle.
            current_angle = if target_angle > current_angle {
                current_angle + i as f64 * step_size
            } else {
                current_angle - i as f64 * step_size
            };

            // Write the current angle.
            self.write(current_angle).await?;

            // Sleep for the computed amount of time.
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

        // Update the current angle.
        _ = self.angle_sender.send(angle);

        // Return success.
        Ok(())
    }
}
