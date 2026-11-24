#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parsed_values_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
                .map(|(_, n)| u32::from(n))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
