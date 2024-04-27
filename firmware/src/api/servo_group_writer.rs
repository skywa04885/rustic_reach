use com::proto::{RpcPose, RpcPoseChange};
use pca9685_servo::servo::writer::ServoWriter;
use tokio::try_join;
use tonic::Status;

pub(crate) struct ServoGroupWriter {
    s01_w: ServoWriter,
    s02_w: ServoWriter,
    s03_w: ServoWriter,
    s04_w: ServoWriter,
    s05_w: ServoWriter,
    s06_w: ServoWriter,
}

impl ServoGroupWriter {
    pub(super) fn new(
        s01_w: ServoWriter,
        s02_w: ServoWriter,
        s03_w: ServoWriter,
        s04_w: ServoWriter,
        s05_w: ServoWriter,
        s06_w: ServoWriter,
    ) -> Self {
        Self {
            s01_w,
            s02_w,
            s03_w,
            s04_w,
            s05_w,
            s06_w,
        }
    }

    pub(crate) async fn write_rpc_pose_change(
        &mut self,
        pose_change: RpcPoseChange,
    ) -> Result<(), Status> {
        let ServoGroupWriter {
            s01_w,
            s02_w,
            s03_w,
            s04_w,
            s05_w,
            s06_w,
        } = self;

        let RpcPoseChange { new_pose, duration } = pose_change;

        let RpcPose {
            angle0,
            angle1,
            angle2,
            angle3,
            angle4,
            angle5,
        } = new_pose.ok_or_else(|| Status::invalid_argument("new_pose must be provided"))?;

        if let Err(error) = try_join!(
            s01_w.write_with_duration(angle0, duration),
            s02_w.write_with_duration(angle1, duration),
            s03_w.write_with_duration(angle2, duration),
            s04_w.write_with_duration(angle3, duration),
            s05_w.write_with_duration(angle4, duration),
            s06_w.write_with_duration(angle5, duration),
        ) {
            return Err(Status::internal(error.to_string()));
        }

        Ok(())
    }
}
