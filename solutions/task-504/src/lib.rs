// rust-track: handwritten task-504
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 504;

pub fn columns(rows: &[(i8, i8)]) -> (Vec<i8>, Vec<i8>) {
    rows.iter().copied().unzip()
}
