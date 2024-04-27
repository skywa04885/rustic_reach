use pca9685_servo::servo::{reader::ServoReader, writer::ServoWriter};

use self::{
    servo_group_reader::{ServoGroupReader, ServoGroupReaderHandle, ServoGroupReaderTask},
    servo_group_writer::ServoGroupWriter,
};

pub(crate) mod servo_group_reader;
pub(crate) mod servo_group_writer;

pub(crate) struct ServoGroup;

impl ServoGroup {
    pub(crate) fn new(
        (s01_w, s01_r): (ServoWriter, ServoReader),
        (s02_w, s02_r): (ServoWriter, ServoReader),
        (s03_w, s03_r): (ServoWriter, ServoReader),
        (s04_w, s04_r): (ServoWriter, ServoReader),
        (s05_w, s05_r): (ServoWriter, ServoReader),
        (s06_w, s06_r): (ServoWriter, ServoReader),
    ) -> (
        ServoGroupWriter,
        ServoGroupReaderHandle,
        ServoGroupReaderTask,
    ) {
        let writer = ServoGroupWriter::new(s01_w, s02_w, s03_w, s04_w, s05_w, s06_w);

        let (reader_task, reader_handle) =
            ServoGroupReader::new(s01_r, s02_r, s03_r, s04_r, s05_r, s06_r);

        (writer, reader_handle, reader_task)
    }
}
