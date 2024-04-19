/// Computes the duty cycle based on the start and end duty cycles, start and end angles, and the current angle.
///
/// # Arguments
///
/// * `start_duty_cycle` - The starting duty cycle.
/// * `end_duty_cycle` - The ending duty cycle.
/// * `start_angle` - The starting angle.
/// * `end_angle` - The ending angle.
/// * `angle` - The current angle.
///
/// # Returns
///
/// The computed duty cycle.
pub(crate) fn compute_duty_cycle(
    start_duty_cycle: f64,
    end_duty_cycle: f64,
    start_angle: f64,
    end_angle: f64,
    angle: f64,
) -> f64 {
    start_duty_cycle + (end_duty_cycle - start_duty_cycle) * (angle / (end_angle - start_angle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_duty_cycle() {
        // Define the input values
        let start_duty_cycle = 0.0;
        let end_duty_cycle = 1.0;
        let start_angle = 0.0;
        let end_angle = 180.0;
        let angle = 90.0;

        // Call the function under test
        let result = compute_duty_cycle(
            start_duty_cycle,
            end_duty_cycle,
            start_angle,
            end_angle,
            angle,
        );

        // Define the expected result
        let expected_result = 0.5;

        // Assert that the result matches the expected result
        assert_eq!(result, expected_result);
    }
}
