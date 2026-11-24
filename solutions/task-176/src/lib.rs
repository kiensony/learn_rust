// rust-track: handwritten task-176
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 176;

pub fn override_code_3(first: u16, second: u16) -> bool {
    override_code_3_impl(first, second)
}

fn override_code_3_impl(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value == 3 || value == 5
}
