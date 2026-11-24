// rust-track: handwritten task-1103
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1103;

pub trait Meter {
    fn charge(&self) -> u8;
}
impl Meter for u8 {
    fn charge(&self) -> u8 {
        *self
    }
}
pub fn cost(meter: &dyn Meter) -> u8 {
    meter.charge()
}
