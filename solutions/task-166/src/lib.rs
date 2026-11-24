// rust-track: handwritten task-166
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 166;

pub fn override_code_2(value: u16) -> bool {
    override_code_2_impl(value)
}

fn override_code_2_impl(value: u16) -> bool {
    value == 2 || value == 4
}
