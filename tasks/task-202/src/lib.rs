#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn comfort_zone_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    let value = value.saturating_sub(1);
    (10..=12).contains(&value)
}
