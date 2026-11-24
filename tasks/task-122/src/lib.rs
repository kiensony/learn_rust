#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn price_in_cents_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value.saturating_sub(1)) * 3
}
