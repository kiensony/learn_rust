// rust-track: handwritten task-2773
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2773;

pub fn repeated_keys_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .split(';')
            .filter_map(|field| {
                let (key, raw) = field.split_once('=')?;
                if key.is_empty()
                    || !key.bytes().all(|b| b.is_ascii_alphabetic())
                    || !raw.bytes().all(|b| b.is_ascii_digit())
                {
                    return None;
                }
                Some((key, raw.parse::<u8>().ok()?))
            })
            .scan(std::collections::BTreeSet::new(), |seen, (k, n)| {
                Some((seen.insert(k), n))
            })
            .filter(|(fresh, _)| !fresh)
            .map(|(_, n)| u32::from(n))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
