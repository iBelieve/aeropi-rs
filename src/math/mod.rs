use na::Vector3;

pub type Vec3 = Vector3<f64>;

pub fn twos_comp_combine(msb: u8, lsb: u8) -> i16 {
    let twos_comp: i32 = 256 * (msb as i32) + lsb as i32;

    if twos_comp >= 32768 {
        (twos_comp - 65536) as i16
    } else {
        twos_comp as i16
    }
}
