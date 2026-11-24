#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn release_line_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
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
                if a == 0 || parts.next().is_some() {
                    return None;
                }
                Some(a * 256 + b)
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
