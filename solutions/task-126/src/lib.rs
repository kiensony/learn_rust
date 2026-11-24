// rust-track: handwritten task-126
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 126;

pub fn saturating_debit_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value.saturating_sub(3))
}
