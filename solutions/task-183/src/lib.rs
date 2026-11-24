// rust-track: handwritten task-183
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 183;

pub fn alarm_band_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value < 5 || value > 7
}
