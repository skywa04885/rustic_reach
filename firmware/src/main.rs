use pose::MyArmService;

pub(crate) mod pose;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let my_arm_service = MyArmService {};

    Ok(())
    /*
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0b100_0000).unwrap();

    // Create the device and perform a software reset.
    let mut device = Device::new(i2c, 0b100_0000);
    device.software_reset().await.unwrap();

    // Create the driver and set the oscillator clock frequency and update rate.
    let mut driver = Driver::builder(device)
        .with_osc_clock(25_000_000)
        .with_update_rate(50)
        .build()
        .unwrap();

    // Wake the PCA9685 device from sleep mode.
    driver.wake().await.unwrap();

    // Wrap the driver.
    let driver = Arc::new(Mutex::new(driver));

    // Create the servo.
    let servo_settings = ServoSettings::new();
    let mut servo = Servo::new(Channel::new(driver, 0_u8), servo_settings, 0_f64);
    servo.write_with_speed(120.0, 60.0).await.unwrap();
     */
}
