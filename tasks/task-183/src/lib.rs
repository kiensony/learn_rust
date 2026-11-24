#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn alarm_band_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    let value = value.saturating_sub(1);
    value < 5 || value > 7
}
