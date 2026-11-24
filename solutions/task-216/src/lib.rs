// rust-track: handwritten task-216
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 216;

pub fn doubling_steps_2(value: u8) -> u32 {
    doubling_steps_2_impl(value)
}

fn doubling_steps_2_impl(value: u8) -> u32 {
    let (mut x, mut steps) = (2, 0);
    while x < u32::from(value) {
        x *= 2;
        steps += 1;
    }
    steps
}
