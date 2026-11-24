// rust-track: handwritten task-073
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 73;

pub fn complete_rows_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) / 3
}
