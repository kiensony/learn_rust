// rust-track: handwritten task-2784
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2784;

pub fn parsed_defaults_peaks(input: &str) -> Vec<u32> {
    let samples = observe_parsed_defaults(input);
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

fn observe_parsed_defaults(input: &str) -> Vec<u32> {
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
        .filter(|(_, n)| *n == 0)
        .map(|(k, _)| k.len() as u32)
        .collect()
}
