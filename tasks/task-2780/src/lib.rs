#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn repeated_keys_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .split(';')
            .take(2)
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
    samples.into_iter().rev().collect()
}
