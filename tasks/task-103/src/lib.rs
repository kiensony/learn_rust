#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn complete_rows_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value.saturating_sub(1)) / 10
}
