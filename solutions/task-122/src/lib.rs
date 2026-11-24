// rust-track: handwritten task-122
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 122;

pub fn price_in_cents_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) * 3
}
