use std::sync::Arc;

use com::proto::{
    rpc_servo_driver_api_server::RpcServoDriverApi, RpcMultiPoseChangeRequest,
    RpcMultiPoseChangeResponse, RpcPose, RpcPoseChange, RpcPoseChangeRequest,
    RpcPoseChangeResponse, RpcPoseStreamRequest,
};
use pca9685::{device::Device, Channel, Driver};
use pca9685_servo::{Servo, ServoSettingsProfiles};
use rppal::{
    gpio::Gpio,
    i2c::I2c,
};
use tokio::sync::{Mutex, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
/// Represents a collection of servos.
struct Servos {
    servo01: Servo,
    servo02: Servo,
    servo03: Servo,
    servo04: Servo,
    servo05: Servo,
    servo06: Servo,
}

impl Servos {
    /// Creates a new instance of `Servos`.
    ///
    /// # Arguments
    ///
    /// * `servo01` - The first servo.
    /// * `servo02` - The second servo.
    /// * `servo03` - The third servo.
    /// * `servo04` - The fourth servo.
    /// * `servo05` - The fifth servo.
    /// * `servo06` - The sixth servo.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Servos`.
    pub fn new(
        servo01: Servo,
        servo02: Servo,
        servo03: Servo,
        servo04: Servo,
        servo05: Servo,
        servo06: Servo,
    ) -> Self {
        Self {
            servo01,
            servo02,
            servo03,
            servo04,
            servo05,
            servo06,
        }
    }

    /// Writes the pose change to the servos.
    ///
    /// # Arguments
    ///
    /// * `pose_change` - The pose change to be written.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the write operation is successful, otherwise returns an error of type `Status`.
    pub async fn write_rpc_pose_change(
        &mut self,
        pose_change: RpcPoseChange,
    ) -> Result<(), Status> {
        // Extract the new pose and duration from the RpcPoseChange struct
        let RpcPoseChange { new_pose, duration } = pose_change;

        // Check if the new_pose field is Some
        if let Some(RpcPose {
            angle0,
            angle1,
            angle2,
            angle3,
            angle4,
            angle5,
        }) = new_pose
        {
            // Write the new angles to each servo with the specified duration
            let servo01_write = self.servo01.write_with_duration(angle0, duration);
            let servo02_write = self.servo02.write_with_duration(angle1, duration);
            let servo03_write = self.servo03.write_with_duration(angle2, duration);
            let servo04_write = self.servo04.write_with_duration(angle3, duration);
            let servo05_write = self.servo05.write_with_duration(angle4, duration);
            let servo06_write = self.servo06.write_with_duration(angle5, duration);

            // Use tokio::select! macro to concurrently wait for the write operations to complete
            if let Err(_) = tokio::select! {
                result = servo01_write => result,
                result = servo02_write => result,
                result = servo03_write => result,
                result = servo04_write => result,
                result = servo05_write => result,
                result = servo06_write => result,
            } {
                // If any of the write operations fail, return an internal error
                return Err(Status::internal("Failed to write to one or more servos"));
            }

            // If all write operations are successful, return Ok(())
            Ok(())
        } else {
            // If new_pose field is None, return an invalid argument error
            Err(Status::invalid_argument("new_pose must be provided"))
        }
    }
}

pub struct ServoDriverApi {
    servos: RwLock<Servos>,
}

impl ServoDriverApi {
    /// Creates a new instance of `ServoDriverApi`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ServoDriverApi` instance or a boxed `dyn std::error::Error` if an error occurs.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
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
        let servo01_settings = ServoSettingsProfiles::s06nf_01();
        let mut servo01 = Servo::new(
            Channel::new(driver.clone(), 0_u8),
            servo01_settings,
            0.0_f64,
        );
        servo01.write(0.0).await.unwrap();

        let servo02_settings = ServoSettingsProfiles::s06nf_02();
        let mut servo02 = Servo::new(
            Channel::new(driver.clone(), 1_u8),
            servo02_settings,
            0.0_f64,
        );
        servo02.write(0.0).await.unwrap();

        let servo03_settings = ServoSettingsProfiles::s06nf_03();
        let mut servo03 = Servo::new(
            Channel::new(driver.clone(), 2_u8),
            servo03_settings,
            0.0_f64,
        );
        servo03.write(0.0).await.unwrap();

        let servo04_settings = ServoSettingsProfiles::s06nf_04();
        let mut servo04 = Servo::new(
            Channel::new(driver.clone(), 3_u8),
            servo04_settings,
            0.0_f64,
        );
        servo04.write(0.0).await.unwrap();

        let servo05_settings = ServoSettingsProfiles::s06nf_05();
        let mut servo05 = Servo::new(
            Channel::new(driver.clone(), 4_u8),
            servo05_settings,
            0.0_f64,
        );
        servo05.write(0.0).await.unwrap();

        let servo06_settings = ServoSettingsProfiles::s06nf_06();
        let mut servo06 = Servo::new(
            Channel::new(driver.clone(), 5_u8),
            servo06_settings,
            0.0_f64,
        );
        servo06.write(0.0).await.unwrap();

        // Create the Servos instance
        let servos = Servos::new(servo01, servo02, servo03, servo04, servo05, servo06);

        // Return the ServoDriverApi instance
        Ok(Self {
            servos: RwLock::new(servos),
        })
    }
}

#[tonic::async_trait]
impl RpcServoDriverApi for ServoDriverApi {
    type PoseStreamStream = ReceiverStream<Result<RpcPose, Status>>;

    async fn change_pose(
        &self,
        request: Request<RpcPoseChangeRequest>,
    ) -> Result<Response<RpcPoseChangeResponse>, Status> {
        let RpcPoseChangeRequest { pose_change } = request.into_inner();

        if let Some(pose_change) = pose_change {
            let mut servos = self.servos.write().await;

            servos.write_rpc_pose_change(pose_change).await?;

            Ok(Response::new(RpcPoseChangeResponse {}))
        } else {
            return Err(Status::invalid_argument("pose_change must be provided"));
        }
    }

    async fn multi_change_pose(
        &self,
        request: Request<RpcMultiPoseChangeRequest>,
    ) -> Result<Response<RpcMultiPoseChangeResponse>, Status> {
        let servos = self.servos.write().await;

        unimplemented!()
    }

    async fn pose_stream(
        &self,
        request: Request<RpcPoseStreamRequest>,
    ) -> Result<tonic::Response<Self::PoseStreamStream>, Status> {
        unimplemented!()
    }
}
