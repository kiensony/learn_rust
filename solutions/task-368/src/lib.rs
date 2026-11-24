// rust-track: handwritten task-368
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 368;

pub fn label_info(label: &String) -> (usize, bool) {
    (label.len(), label.is_ascii())
}
