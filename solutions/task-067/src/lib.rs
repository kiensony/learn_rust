// rust-track: handwritten task-067
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 67;

pub fn reverse_a_reading_2(value: u16) -> u32 {
    reverse_a_reading_2_impl(value)
}

fn reverse_a_reading_2_impl(value: u16) -> u32 {
    (65535 - u32::from(value)) / 2
}
