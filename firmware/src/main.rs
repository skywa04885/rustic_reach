use pca9685::{device::Device, Driver};
use rppal::i2c::I2c;

#[tokio::main]
async fn main() {
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

    
    driver.write_channel_duty_cycle(1, 0.764).unwrap();
}
