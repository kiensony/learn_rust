// rust-track: handwritten task-152
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 152;

pub fn price_in_cents_10(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value) * 10
}
