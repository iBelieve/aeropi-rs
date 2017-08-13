use std::fs::File;
use math::Vec3;
use serde_yaml;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AccelerometerCalibration {
    pub offsets: Vec3
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Calibration {
    pub accelerometer: AccelerometerCalibration
}

impl Calibration {
    pub fn load() -> Option<Calibration> {
        File::open("calibration.yml").ok()
            .and_then(|file| serde_yaml::from_reader(&file).ok())
    }

    pub fn save(&self) {
        let file = File::create("calibration.yml")
            .expect("Unable to create calibration file");
        serde_yaml::to_writer(&file, self)
            .expect("Unable to write calibration data");
    }
}
