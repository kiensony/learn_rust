#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 212;

pub fn first_even(values: &[u8]) -> Option<u8> {
    let mut result = None;
    'scan: for &n in values {
        if n % 2 == 0 {
            result = Some(n);
            break 'search;
        }
    }
    result
}
