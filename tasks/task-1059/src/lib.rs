#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1059;

pub trait Sensor {
    fn read(&self) -> u8;
}
pub struct Dial(pub u8);
impl Sensor for Dial {
    fn read(&mut self) -> u8 {
        self.0
    }
}
