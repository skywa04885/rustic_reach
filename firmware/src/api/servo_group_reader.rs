use std::time::Duration;

use com::proto::RpcPose;
use pca9685_servo::servo::reader::ServoReader;
use thiserror::Error;
use tokio::select;
use tokio_util::sync::CancellationToken;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Task closed")]
    TaskClosedError,
    #[error("Cancelled")]
    Cancelled,
    #[error("Servo reader error: {0}")]
    ServoReaderError(#[from] pca9685_servo::servo::reader::Error),
}

pub(crate) struct ServoGroupReader;

impl ServoGroupReader {
    pub(crate) fn new(
        s01_r: ServoReader,
        s02_r: ServoReader,
        s03_r: ServoReader,
        s04_r: ServoReader,
        s05_r: ServoReader,
        s06_r: ServoReader,
    ) -> (ServoGroupReaderTask, ServoGroupReaderHandle) {
        let (pose_sender, pose_receiver) = tokio::sync::broadcast::channel(64_usize);

        let task = ServoGroupReaderTask::new(s01_r, s02_r, s03_r, s04_r, s05_r, s06_r, pose_sender);
        let handle = ServoGroupReaderHandle::new(pose_receiver);

        (task, handle)
    }
}

pub(crate) struct ServoGroupReaderTask {
    s01_r: ServoReader,
    s02_r: ServoReader,
    s03_r: ServoReader,
    s04_r: ServoReader,
    s05_r: ServoReader,
    s06_r: ServoReader,
    pose_sender: tokio::sync::broadcast::Sender<RpcPose>,
}

impl ServoGroupReaderTask {
    const SLEEP_DURATION: Duration = Duration::from_millis(20);

    pub(self) fn new(
        s01_r: ServoReader,
        s02_r: ServoReader,
        s03_r: ServoReader,
        s04_r: ServoReader,
        s05_r: ServoReader,
        s06_r: ServoReader,
        pose_sender: tokio::sync::broadcast::Sender<RpcPose>,
    ) -> Self {
        Self {
            s01_r,
            s02_r,
            s03_r,
            s04_r,
            s05_r,
            s06_r,
            pose_sender,
        }
    }

    pub(crate) async fn run(&mut self) -> Result<(), Error> {
        loop {
            select! {
                x = self.s01_r.wait_for_angle_to_change() => x?,
                x = self.s02_r.wait_for_angle_to_change() => x?,
                x = self.s03_r.wait_for_angle_to_change() => x?,
                x = self.s04_r.wait_for_angle_to_change() => x?,
                x = self.s05_r.wait_for_angle_to_change() => x?,
                x = self.s06_r.wait_for_angle_to_change() => x?,
            };

            tokio::time::sleep(Self::SLEEP_DURATION).await;

            if let Err(_) = self.pose_sender.send(RpcPose {
                angle0: self.s01_r.read_angle(),
                angle1: self.s02_r.read_angle(),
                angle2: self.s03_r.read_angle(),
                angle3: self.s04_r.read_angle(),
                angle4: self.s05_r.read_angle(),
                angle5: self.s06_r.read_angle(),
            }) {
                break;
            }
        }

        // Return success.
        Ok(())
    }
}

pub(crate) struct ServoGroupReaderHandle {
    pose_receiver: tokio::sync::broadcast::Receiver<RpcPose>,
}

impl ServoGroupReaderHandle {
    pub(self) fn new(pose_receiver: tokio::sync::broadcast::Receiver<RpcPose>) -> Self {
        Self { pose_receiver }
    }

    pub(crate) async fn recv_pose(&mut self) -> Result<RpcPose, Error> {
        if let Ok(pose) = self.pose_receiver.recv().await {
            Ok(pose)
        } else {
            Err(Error::TaskClosedError)
        }
    }
}
