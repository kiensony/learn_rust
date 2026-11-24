#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1010;

#[derive(Debug, PartialEq)]
pub struct Ordered<A, B>(pub A, pub B);
pub fn pair(x: u16, y: u16) -> Ordered<u16> {
    Ordered(x, y)
}
