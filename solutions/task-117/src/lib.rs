// rust-track: handwritten task-117
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 117;

pub fn fit_a_byte_2(value: u16) -> u32 {
    fit_a_byte_2_impl(value)
}

fn fit_a_byte_2_impl(value: u16) -> u32 {
    (u32::from(value) + 2).min(255)
}
