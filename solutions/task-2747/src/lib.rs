// rust-track: handwritten task-2747
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2747;

pub fn cli_extensions_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .map(std::path::Path::new)
            .filter_map(|p| p.extension())
            .map(|e| e.len() as u32)
            .collect()
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
