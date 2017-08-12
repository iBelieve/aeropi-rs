use math::Vec3;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Calibration {
    pub accelerometer: AccelerometerCalibration
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AccelerometerCalibration {
    pub offsets: Vec3
}
