// rust-track: handwritten task-855
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 855;

pub mod scale {
    pub fn quarter(n: u32) -> u32 {
        n / 4
    }
}
pub fn price(raw: u32) -> u32 {
    scale::quarter(raw)
}
