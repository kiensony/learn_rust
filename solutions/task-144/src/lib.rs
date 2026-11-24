// rust-track: handwritten task-144
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 144;

pub fn round_up_8(start: u16, end: u16) -> u32 {
    round_up_8_impl(start, end)
}

fn round_up_8_impl(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value).div_ceil(8) * 8
}
