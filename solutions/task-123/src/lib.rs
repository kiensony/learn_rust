// rust-track: handwritten task-123
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 123;

pub fn round_down_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) / 3 * 3
}
