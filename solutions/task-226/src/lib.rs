// rust-track: handwritten task-226
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 226;

pub fn doubling_steps_3(first: u8, second: u8) -> u32 {
    doubling_steps_3_impl(first, second)
}

fn doubling_steps_3_impl(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    let (mut x, mut steps) = (3, 0);
    while x < u32::from(value) {
        x *= 2;
        steps += 1;
    }
    steps
}
