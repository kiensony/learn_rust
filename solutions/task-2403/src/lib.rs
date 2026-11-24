// rust-track: handwritten task-2403
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2403;

#[repr(u8)]
pub enum Tag {
    Start = 0,
    Data = 1,
    End = 255,
}
pub fn encode(t: Tag) -> u8 {
    t as u8
}
