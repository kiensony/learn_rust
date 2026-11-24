// rust-track: handwritten task-1697
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1697;

pub fn minor_version_run_ledger(input: &str) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        input
            .split_ascii_whitespace()
            .filter_map(|w| {
                let mut parts = w.split('.');
                let mut next = || {
                    let p = parts.next()?;
                    if !p.bytes().all(|x| x.is_ascii_digit()) {
                        return None;
                    }
                    Some(u32::from(p.parse::<u8>().ok()?))
                };
                let (a, b, c) = (next()?, next()?, next()?);
                let _ = (a, b, c);
                if parts.next().is_some() {
                    return None;
                }
                Some(b)
            })
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
