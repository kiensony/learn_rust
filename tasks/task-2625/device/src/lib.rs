#![no_std]

pub fn convert(x: u8) -> u8 {
    (x.count_ones() % 2) as u8
}
