// rust-track: handwritten task-154
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 154;

pub fn round_up_10(requested: u16, available: u16) -> u32 {
    round_up_10_impl(requested, available)
}

fn round_up_10_impl(requested: u16, available: u16) -> u32 {
    let value = requested.min(available);
    u32::from(value).div_ceil(10) * 10
}
