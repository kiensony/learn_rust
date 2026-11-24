#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 360;

pub fn letter_count(name: &String) -> usize {
    let owned = *name;
    owned.chars().count()
}
