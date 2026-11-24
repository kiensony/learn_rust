// rust-track: handwritten task-1077
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1077;

pub fn consume_first<I: Iterator>(mut items: I) -> Option<I::Item> {
    items.next()
}
