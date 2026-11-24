// rust-track: handwritten task-127
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 127;

pub fn fit_a_byte_3(first: u16, second: u16) -> u32 {
    fit_a_byte_3_impl(first, second)
}

fn fit_a_byte_3_impl(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    (u32::from(value) + 3).min(255)
}
