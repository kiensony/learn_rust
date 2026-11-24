// rust-track: handwritten task-2247
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2247;

pub fn borrowed_offsets_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = { input.match_indices('\n').map(|(i, _)| i as u32).collect() };
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
