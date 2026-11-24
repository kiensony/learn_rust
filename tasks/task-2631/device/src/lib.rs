#![no_std]

pub fn convert(x: u8) -> u8 {
    x.saturating_mul(2)
}
