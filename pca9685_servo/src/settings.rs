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
    pub const DEFAULT_START_ANGLE: f64 = -90_f64;
    pub const DEFAULT_END_ANGLE: f64 = 90_f64;
    pub const DEFAULT_START_DUTY_CYCLE: f64 = 0.025_f64;
    pub const DEFAULT_END_DUTY_CYCLE: f64 = 0.125_f64;

    /// Creates a new `ServoSettings` instance with default values.
    ///
    /// # Returns
    ///
    /// The new `ServoSettings` instance.
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
