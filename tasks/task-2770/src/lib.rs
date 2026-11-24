#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parsed_key_lengths_reverse(input: &str) -> Vec<u32> {
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
            .map(|(k, _)| k.len() as u32)
            .collect()
    };
    let _ = samples;
    Vec::new()
}
