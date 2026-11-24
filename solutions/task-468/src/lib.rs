// rust-track: handwritten task-468
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 468;

pub fn shift(lamps: [u8; 3]) -> [u8; 3] {
    let [a, b, c] = lamps;
    [b, c, a]
}
