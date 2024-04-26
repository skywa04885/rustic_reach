use std::sync::Arc;

use pca9685::{device::Device, Channel, Driver};
use pca9685_servo::{Servo, ServoSettings, ServoSettingsProfiles};
use pose::MyArmService;
use rppal::{gpio::Gpio, i2c::I2c};
use tokio::{signal::ctrl_c, sync::Mutex};

pub(crate) mod pose;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let my_arm_service = MyArmService {};

    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0b100_0000).unwrap();

    // Create the device and perform a software reset.
    let mut device = Device::new(i2c, 0b100_0000);
    device.software_reset().await.unwrap();

    let oe_pin = Gpio::new().unwrap().get(23).unwrap().into_output();

    // Create the driver and set the oscillator clock frequency and update rate.
    let mut driver = Driver::builder(device, oe_pin)
        .with_osc_clock(26_600_000)
        .with_update_rate(50)
        .build()
        .unwrap();

    // Wake the PCA9685 device from sleep mode.
    driver.wake().await.unwrap();

    // Wrap the driver.
    let driver = Arc::new(Mutex::new(driver));

    // Create servo01.
    let servo01_settings = ServoSettingsProfiles::s06nf_01();
    let mut servo01 = Servo::new(Channel::new(driver.clone(), 0_u8), servo01_settings, 0.0_f64);
    servo01.write(0.0).await.unwrap();

    // Create servo02.
    let servo02_settings = ServoSettingsProfiles::s06nf_02();
    let mut servo02 = Servo::new(Channel::new(driver.clone(), 1_u8), servo02_settings, 0.0_f64);
    servo02.write(0.0).await.unwrap();

    // Create servo03.
    let servo03_settings = ServoSettingsProfiles::s06nf_03();
    let mut servo03 = Servo::new(Channel::new(driver.clone(), 2_u8), servo03_settings, 0.0_f64);
    servo03.write(0.0).await.unwrap();

    // Create servo04.
    let servo04_settings = ServoSettingsProfiles::s06nf_04();
    let mut servo04 = Servo::new(Channel::new(driver.clone(), 3_u8), servo04_settings, 0.0_f64);
    servo04.write(0.0).await.unwrap();

    // Create servo05.
    let servo05_settings = ServoSettingsProfiles::s06nf_05();
    let mut servo05 = Servo::new(Channel::new(driver.clone(), 4_u8), servo05_settings, 0.0_f64);
    servo05.write(0.0).await.unwrap();

    // Create servo06.
    let servo06_settings = ServoSettingsProfiles::s06nf_06();
    let mut servo06 = Servo::new(Channel::new(driver.clone(), 5_u8), servo06_settings, 0.0_f64);
    servo06.write(0.0).await.unwrap();

    ctrl_c().await;

    Ok(())
}
