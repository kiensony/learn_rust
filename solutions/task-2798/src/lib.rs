// rust-track: handwritten task-2798
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2798;

pub fn repeated_keys_rolling_load(input: &str, width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
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
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
