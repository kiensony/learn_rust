// rust-track: handwritten task-116
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 116;

pub fn saturating_debit_2(value: u16) -> u32 {
    u32::from(value.saturating_sub(2))
}
