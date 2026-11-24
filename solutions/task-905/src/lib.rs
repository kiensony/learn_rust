// rust-track: handwritten task-905
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 905;

pub fn quote(discount: impl Fn(u16, u16) -> u16, list: u16, rebate: u16) -> u16 {
    discount(list, rebate)
}
