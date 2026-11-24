#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 410;

pub fn repeat_first(values: &mut Vec<u8>) -> Option<u8> {
    let first = values.first()?;
    values.push(*first);
    Some(*first)
}
