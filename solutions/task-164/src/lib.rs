// rust-track: handwritten task-164
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 164;

pub fn alarm_band_2(value: u16) -> bool {
    value < 2 || value > 4
}
