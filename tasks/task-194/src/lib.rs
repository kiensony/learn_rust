#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn even_batch_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    let value = value.saturating_sub(1);
    value >= 8 && value % 2 == 0
}
