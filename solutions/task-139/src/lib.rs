// rust-track: handwritten task-139
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 139;

pub fn nearest_group_5(issued: u16, returned: u16) -> u32 {
    nearest_group_5_impl(issued, returned)
}

fn nearest_group_5_impl(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    ((u32::from(value) + 2) / 5) * 5
}
