#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parsed_values_bounds(input: &str) -> Option<(u32, u32)> {
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
            .map(|(_, n)| u32::from(n))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
