// rust-track: handwritten task-750
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 750;

pub fn first_ok(samples: &[Result<i16, ()>]) -> Option<i16> {
    samples
        .iter()
        .find_map(|sample| sample.as_ref().ok().copied())
}
