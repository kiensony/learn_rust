// rust-track: handwritten task-253
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 253;

pub fn running_deposits_10(requested: u8, available: u8) -> u32 {
    running_deposits_10_impl(requested, available)
}

fn running_deposits_10_impl(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    (1..=u32::from(value)).map(|day| day * 10).sum()
}
