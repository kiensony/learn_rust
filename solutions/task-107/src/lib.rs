// rust-track: handwritten task-107
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 107;

pub fn label_size(label: &str) -> usize {
    let label = label.trim();
    let label = label.len();
    label
}
