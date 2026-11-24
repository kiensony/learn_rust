// rust-track: handwritten task-203
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 203;

pub fn alarm_band_10(requested: u16, available: u16) -> bool {
    alarm_band_10_impl(requested, available)
}

fn alarm_band_10_impl(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value < 10 || value > 12
}
