#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn even_batch_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    let value = value.saturating_sub(1);
    value >= 3 && value % 2 == 0
}
