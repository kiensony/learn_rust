// rust-track: handwritten task-246
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 246;

pub fn doubling_steps_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let (mut x, mut steps) = (8, 0);
    while x < u32::from(value) {
        x *= 2;
        steps += 1;
    }
    steps
}
