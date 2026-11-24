// rust-track: handwritten task-652
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 652;

#[derive(Debug, PartialEq, Eq)]
pub struct Stock {
    pub units: u16,
    pub capacity: u16,
}

pub fn restock(original: Stock, amount: u16) -> Stock {
    Stock {
        units: original.units.saturating_add(amount).min(original.capacity),
        ..original
    }
}
