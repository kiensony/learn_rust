// rust-track: handwritten task-155
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 155;

pub fn top_up_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    (10 - u32::from(value) % 10) % 10
}
