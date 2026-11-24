#![no_std]

pub fn convert(x: u8) -> u8 {
    x.rotate_right(1)
}
