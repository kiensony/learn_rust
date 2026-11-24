// rust-track: handwritten task-2547
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2547;

pub fn abi_flags_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        extern "C" fn transform(x: u8) -> u32 {
            u32::from(x != 0)
        }
        let callback: extern "C" fn(u8) -> u32 = transform;
        input.iter().map(|&x| callback(x)).collect()
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
