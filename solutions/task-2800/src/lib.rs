// rust-track: handwritten task-2800
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2800;

pub fn final_settings_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
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
            .collect::<std::collections::BTreeMap<_, _>>()
            .into_values()
            .map(u32::from)
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
