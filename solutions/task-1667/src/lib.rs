// rust-track: handwritten task-1667
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1667;

pub fn minor_version_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_minor_version(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_minor_version(input: &str) -> Vec<u32> {
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
}
