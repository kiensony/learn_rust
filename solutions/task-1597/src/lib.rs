// rust-track: handwritten task-1597
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1597;

pub fn feature_padding_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    let mut runs: Vec<(u32, usize)> = Vec::new();
    for value in observations {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
