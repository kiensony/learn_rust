// rust-track: handwritten task-156
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 156;

pub fn saturating_debit_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value.saturating_sub(10))
}
