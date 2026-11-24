// rust-track: handwritten task-173
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 173;

pub fn alarm_band_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value < 3 || value > 5
}
