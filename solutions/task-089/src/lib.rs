// rust-track: handwritten task-089
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 89;

pub fn a_full_tray_5(issued: u16, returned: u16) -> bool {
    a_full_tray_5_impl(issued, returned)
}

fn a_full_tray_5_impl(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value % 5 == 0
}
