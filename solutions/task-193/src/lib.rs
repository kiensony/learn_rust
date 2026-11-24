// rust-track: handwritten task-193
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 193;

pub fn alarm_band_8(start: u16, end: u16) -> bool {
    alarm_band_8_impl(start, end)
}

fn alarm_band_8_impl(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value < 8 || value > 10
}
