#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn skip_reserved_seats_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    (1..=u32::from(value.saturating_sub(1)))
        .filter(|i| i % 10 != 0)
        .count() as u32
}
