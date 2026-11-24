// rust-track: handwritten task-318
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 318;

pub fn unpack(package: (String, u16)) -> (u16, String) {
    let (label, quantity) = package;
    (quantity, label)
}
