// rust-track: handwritten task-146
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 146;

pub fn saturating_debit_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value.saturating_sub(8))
}
