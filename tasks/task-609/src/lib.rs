#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 609;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
pub fn locate(east: i16, north: i16) -> Point {
    Point { x: east, x: north }
}
