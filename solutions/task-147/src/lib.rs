// rust-track: handwritten task-147
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 147;

pub fn fit_a_byte_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    (u32::from(value) + 8).min(255)
}
