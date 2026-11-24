#![forbid(unsafe_code)]
pub fn decode(x: u8) -> u8 {
    (x.count_ones() % 2) as u8
}
