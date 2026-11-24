// rust-track: handwritten task-1700
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1700;

pub fn release_line_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
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
                Some(a * 256 + b)
            })
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
