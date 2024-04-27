pub mod reader;
pub mod writer;

use crate::settings::ServoSettings;

use self::{reader::ServoReader, writer::ServoWriter};

pub struct Servo;

impl Servo {
    pub async fn new(channel: pca9685::Channel, settings: ServoSettings, initial_angle: f64) -> Result<(ServoWriter, ServoReader), writer::Error> {
        let (angle_sender, angle_receiver) = tokio::sync::watch::channel(initial_angle);

        let mut servo_writer = ServoWriter::new(channel, settings, angle_sender);
        let servo_reader = ServoReader::new(angle_receiver);

        servo_writer.write(initial_angle).await?;

        Ok((servo_writer, servo_reader))
    }
}