// rust-track: handwritten task-2793
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2793;

pub fn final_settings_bounds(input: &str) -> Option<(u32, u32)> {
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
            .collect::<std::collections::BTreeMap<_, _>>()
            .into_values()
            .map(u32::from)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
