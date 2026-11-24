#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn release_line_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
                    Some(a * 256 + b)
                })
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
