// rust-track: handwritten task-1054
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1054;

pub trait Capacity {
    fn slots(&self) -> u16;
}
pub struct Shelf(pub u16);
impl Capacity for Shelf {
    fn slots(&self) -> u16 {
        self.0
    }
}
