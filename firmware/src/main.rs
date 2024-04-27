use std::sync::Arc;

use api::{
    servo_group_reader::{ServoGroupReaderHandle, ServoGroupReaderTask},
    servo_group_writer::ServoGroupWriter,
    ServoGroup,
};
use com::proto::rpc_servo_writer_api_server::RpcServoWriterApiServer;
use pca9685::{device::Device, Driver};
use pca9685_servo::{servo::Servo, settings::ServoSettings};
use servo_writer_api::ServoWriterApi;
use rppal::{gpio::Gpio, i2c::I2c};
use tokio::{sync::Mutex, try_join};
use tonic::transport::Server;

pub(crate) mod api;
pub(crate) mod servo_writer_api;

pub struct ServoSettingsProfiles;

impl ServoSettingsProfiles {
    pub fn s06nf_01() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.033_f64)
            .with_end_duty_cycle(0.127_f64)
    }

    pub fn s06nf_02() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.021_f64)
            .with_end_duty_cycle(0.127_f64)
    }

    pub fn s06nf_03() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.028_f64)
            .with_end_duty_cycle(0.127_f64)
    }

    pub fn s06nf_04() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.024_f64)
            .with_end_duty_cycle(0.128_f64)
    }

    pub fn s06nf_05() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.026_f64)
            .with_end_duty_cycle(0.104_f64)
    }

    pub fn s06nf_06() -> ServoSettings {
        ServoSettings::new()
            .with_start_angle(-90_f64)
            .with_end_angle(90_f64)
            .with_start_duty_cycle(0.026_f64)
            .with_end_duty_cycle(0.104_f64)
    }
}

async fn create_servo_group() -> Result<
    (
        ServoGroupWriter,
        ServoGroupReaderHandle,
        ServoGroupReaderTask,
    ),
    Box<dyn std::error::Error>,
> {
    // Initialize I2C communication
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(0b100_0000)?;

    // Create a new PCA9685 device
    let mut device = Device::new(i2c, 0b100_0000);
    device.software_reset().await?;

    // Set the OE pin for output enable
    let oe_pin = Gpio::new().unwrap().get(23).unwrap().into_output();

    // Create a new PCA9685 driver
    let mut driver = Driver::builder(device, oe_pin)
        .with_osc_clock(26_600_000)
        .with_update_rate(50)
        .build()
        .unwrap();

    // Wake up the driver
    driver.wake().await.unwrap();

    // Create an Arc-wrapped Mutex for thread-safe access to the driver
    let driver = Arc::new(Mutex::new(driver));

    // Create and initialize each servo
    let s01 = Servo::new(
        pca9685::Channel::new(driver.clone(), 0_u8),
        ServoSettingsProfiles::s06nf_01(),
        0.0_f64,
    ).await?;

    let s02 = Servo::new(
        pca9685::Channel::new(driver.clone(), 1_u8),
        ServoSettingsProfiles::s06nf_02(),
        0.0_f64,
    ).await?;

    let s03 = Servo::new(
        pca9685::Channel::new(driver.clone(), 2_u8),
        ServoSettingsProfiles::s06nf_03(),
        0.0_f64,
    ).await?;

    let s04 = Servo::new(
        pca9685::Channel::new(driver.clone(), 3_u8),
        ServoSettingsProfiles::s06nf_04(),
        0.0_f64,
    ).await?;

    let s05 = Servo::new(
        pca9685::Channel::new(driver.clone(), 4_u8),
        ServoSettingsProfiles::s06nf_05(),
        0.0_f64,
    ).await?;

    let s06 = Servo::new(
        pca9685::Channel::new(driver.clone(), 5_u8),
        ServoSettingsProfiles::s06nf_06(),
        0.0_f64,
    ).await?;

    Ok(ServoGroup::new(s01, s02, s03, s04, s05, s06))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (servo_group_writer, servo_group_reader_handle, mut servo_group_reader_task) = create_servo_group().await?;

    tokio::spawn(async move {
        servo_group_reader_task.run().await.unwrap();
    });

    let servo_writer_api = ServoWriterApi::new(servo_group_writer);
    let servo_writer_api_server = RpcServoWriterApiServer::new(servo_writer_api);

    Server::builder()
        .add_service(servo_writer_api_server)
        .serve("[::1]:50051".parse()?)
        .await?;

    Ok(())
}
