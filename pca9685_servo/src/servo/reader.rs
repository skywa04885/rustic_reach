use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Receiver closed")]
    ReceiverClosed,
}

pub struct ServoReader {
    angle_receiver: tokio::sync::watch::Receiver<f64>,
}

impl ServoReader {
    pub(crate) fn new(angle_receiver: tokio::sync::watch::Receiver<f64>) -> Self {
        Self { angle_receiver }
    }

    pub async fn wait_for_angle_to_change(&mut self) -> Result<(), Error> {
        if let Err(_) = self.angle_receiver.changed().await {
            return Err(Error::ReceiverClosed);
        }

        Ok(())
    }

    pub fn read_angle(&self) -> f64 {
        *self.angle_receiver.borrow()
    }
}
