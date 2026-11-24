// rust-track: handwritten task-163
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 163;

pub fn price(color: bool) -> u8 {
    if color { 5 } else { 2 }
}
