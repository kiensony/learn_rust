// rust-track: handwritten task-188
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 188;

pub fn priority_fee_5(issued: u16, returned: u16) -> u8 {
    priority_fee_5_impl(issued, returned)
}

fn priority_fee_5_impl(issued: u16, returned: u16) -> u8 {
    let value = issued.saturating_sub(returned);
    if value == 0 {
        0
    } else if value <= 7 {
        5
    } else {
        10
    }
}
