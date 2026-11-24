#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 955;

pub fn twice_all(rows: &[u8]) -> u64 {
    rows.map(|&n| 2 * u64::from(n)).sum()
}
