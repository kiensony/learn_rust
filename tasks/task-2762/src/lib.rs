#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parsed_key_lengths_total(input: &str) -> u64 {
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
            .map(|(k, _)| k.len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
