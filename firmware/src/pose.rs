use pca9685_servo::Servo;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;

use com::proto::{
    arm_service_server::ArmService, MovePoseRequest, MovePoseResponse, PoseStreamRequest,
    PoseStreamResponse,
};

pub struct Arm {
    servos: [Servo; 5],
}

impl Arm {
    pub fn write(&mut self, pose: [f64; 5], duration: f64) {
        for (mut servo, angle) in self.servos.iter_mut().zip(pose.iter()) {
            servo.write_with_duration(*angle, duration);
        }
    }
}

pub struct MyArmService {}

impl MyArmService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl ArmService for MyArmService {
    type PoseStreamStream = ReceiverStream<Result<PoseStreamResponse, Status>>;

    async fn move_pose(
        &self,
        request: tonic::Request<MovePoseRequest>,
    ) -> std::result::Result<tonic::Response<MovePoseResponse>, tonic::Status> {
        unimplemented!()
    }

    async fn pose_stream(
        &self,
        request: tonic::Request<PoseStreamRequest>,
    ) -> std::result::Result<tonic::Response<Self::PoseStreamStream>, tonic::Status> {
        unimplemented!()
    }
}
