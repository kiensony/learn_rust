// rust-track: handwritten task-2767
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2767;

pub fn parsed_key_lengths_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_parsed_key_lengths(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_parsed_key_lengths(input: &str) -> Vec<u32> {
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
}
