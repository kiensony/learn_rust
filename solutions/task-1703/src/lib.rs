// rust-track: handwritten task-1703
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1703;

macro_rules! field {
    ($item:expr) => {
        $item.0
    };
}
pub struct Meter(pub u16);
impl Meter {
    pub fn value(&self) -> u16 {
        field!(self)
    }
}
