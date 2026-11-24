// rust-track: handwritten task-1696
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1696;

pub fn major_version_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
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
                Some(a)
            })
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
