use std::fs::{self, File};
use std::io::Write;

use tempfile::tempdir;

use crate::utilities;

pub trait Parcel {
    fn serialize(&self) -> String;
    fn deserialize(&self, data: &str) -> Self;

    fn launch_in_editor(&self) -> Result<Self, String>
    where
        Self: Sized,
    {
        let contents = self.serialize();

        let dir = tempdir().expect("Failed to create temp directory");
        let file_path = dir.path().join("toggl.txt");

        let mut file = File::create_new(file_path.clone()).expect("Failed to create file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write current time-entry to file");

        utilities::open_path_in_editor(&file_path).expect("Failed to open file in editor");
        drop(file);

        let contents = fs::read_to_string(file_path)
            .expect("Failed to read file time-entry editing in editor");

        dir.close().expect("Failed to clear temp directory");

        Ok(self.deserialize(&contents))
    }
}
