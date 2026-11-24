// rust-track: handwritten task-1297
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1297;

pub fn positional_options_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        let mut cmd = std::process::Command::new("report");
        cmd.args(input.split_ascii_whitespace());
        cmd.get_args()
            .filter(|a| !a.to_str().unwrap().starts_with('-'))
            .map(|a| a.len() as u32)
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
