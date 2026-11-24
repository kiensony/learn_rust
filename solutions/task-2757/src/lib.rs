// rust-track: handwritten task-2757
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2757;

pub fn parsed_values_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_parsed_values(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_parsed_values(input: &str) -> Vec<u32> {
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
}
