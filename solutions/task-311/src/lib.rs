// rust-track: handwritten task-311
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 311;

pub fn tag(label: String) -> (String, usize) {
    let owned = label;
    let size = owned.len();
    (owned, size)
}
