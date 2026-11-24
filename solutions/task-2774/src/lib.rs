// rust-track: handwritten task-2774
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2774;

pub fn repeated_keys_peaks(input: &str) -> Vec<u32> {
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
