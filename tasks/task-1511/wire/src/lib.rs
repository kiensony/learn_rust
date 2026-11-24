#![forbid(unsafe_code)]
pub fn decode(x: u8) -> u8 {
    x & 15
}
