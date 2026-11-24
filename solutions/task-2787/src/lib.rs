// rust-track: handwritten task-2787
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2787;

pub fn parsed_defaults_distinct(input: &str) -> Vec<u32> {
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
            .filter(|(_, n)| *n == 0)
            .map(|(k, _)| k.len() as u32)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
