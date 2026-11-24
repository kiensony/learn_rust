// rust-track: handwritten task-256
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 256;

pub fn steps_to(goal: u16) -> u16 {
    let mut total = 0u32;
    let mut step = 0u16;
    while total < u32::from(goal) {
        step += 1;
        total += u32::from(step);
    }
    step
}
