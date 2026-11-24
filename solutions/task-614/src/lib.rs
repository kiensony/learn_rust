// rust-track: handwritten task-614
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 614;

pub struct Parcel {
    pub width: u16,
    pub height: u16,
}
pub fn edge(p: Parcel) -> u16 {
    p.width.max(p.height)
}
