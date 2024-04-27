use com::proto::{
    rpc_servo_writer_api_server::RpcServoWriterApi, RpcMultiPoseChangeRequest,
    RpcMultiPoseChangeResponse, RpcPoseChangeRequest, RpcPoseChangeResponse,
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::api::servo_group_writer::ServoGroupWriter;

pub struct ServoWriterApi {
    servo_group_writer: Mutex<ServoGroupWriter>,
}

impl ServoWriterApi {
    pub fn new(servo_group_writer: ServoGroupWriter) -> Self {
        let servo_group_writer = Mutex::new(servo_group_writer);

        Self { servo_group_writer }
    }
}

#[tonic::async_trait]
impl RpcServoWriterApi for ServoWriterApi {
    async fn change_pose(
        &self,
        request: Request<RpcPoseChangeRequest>,
    ) -> Result<Response<RpcPoseChangeResponse>, Status> {
        let RpcPoseChangeRequest { pose_change } = request.into_inner();

        let pose_change =
            pose_change.ok_or_else(|| Status::invalid_argument("pose_change must be provided"))?;

        let mut servos = self.servo_group_writer.lock().await;

        servos.write_rpc_pose_change(pose_change).await?;

        Ok(Response::new(RpcPoseChangeResponse {}))
    }

    async fn multi_change_pose(
        &self,
        request: Request<RpcMultiPoseChangeRequest>,
    ) -> Result<Response<RpcMultiPoseChangeResponse>, Status> {
        let RpcMultiPoseChangeRequest { pose_changes } = request.into_inner();

        let mut servos = self.servo_group_writer.lock().await;

        for pose_change in pose_changes {
            servos.write_rpc_pose_change(pose_change).await?;
        }

        unimplemented!()
    }
}
