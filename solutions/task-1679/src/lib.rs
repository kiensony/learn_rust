// rust-track: handwritten task-1679
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1679;

pub fn patch_version_longest_run(input: &str) -> u64 {
    let samples = observe_patch_version(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_patch_version(input: &str) -> Vec<u32> {
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
            Some(c)
        })
        .collect()
}
