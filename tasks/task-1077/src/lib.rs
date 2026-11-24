#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn consume_first<I: Iterator>(mut items: I) -> Option<I::Item> {
    items.nth(1)
}
