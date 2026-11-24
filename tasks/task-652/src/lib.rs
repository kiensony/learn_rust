#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct Stock {
    pub units: u16,
    pub capacity: u16,
}

pub fn restock(original: Stock, amount: u16) -> Stock {
    original
}
