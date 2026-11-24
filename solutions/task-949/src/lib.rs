// rust-track: handwritten task-949
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 949;

pub fn rises(values: &[i16]) -> Vec<i16> {
    let mut previous = None;
    values
        .iter()
        .copied()
        .filter(|&n| {
            let rising = previous.is_some_and(|old| n > old);
            previous = Some(n);
            rising
        })
        .collect()
}
