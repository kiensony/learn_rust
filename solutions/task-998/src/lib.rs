// rust-track: handwritten task-998
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 998;

pub fn runs(values: impl IntoIterator<Item = u8>) -> Vec<(u8, usize)> {
    let mut runs: Vec<(u8, usize)> = Vec::new();
    for value in values {
        if let Some(last) = runs.last_mut().filter(|p| p.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
