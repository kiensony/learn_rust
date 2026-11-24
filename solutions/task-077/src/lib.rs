// rust-track: handwritten task-077
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 77;

pub fn reverse_a_reading_3(first: u16, second: u16) -> u32 {
    reverse_a_reading_3_impl(first, second)
}

fn reverse_a_reading_3_impl(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    (65535 - u32::from(value)) / 3
}
