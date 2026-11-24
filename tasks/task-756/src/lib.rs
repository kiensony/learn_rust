#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 756;

pub fn stops(ids: &[u16]) -> Vec<u16> {
    let mut result = ids.iter().copied().collect();
    result.push(0);
    result
}
