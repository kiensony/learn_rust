// rust-track: handwritten task-466
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 466;

pub fn red(rgb: [u8; 3]) -> u8 {
    let [red, _, _] = rgb;
    red
}
