// rust-track: handwritten task-2779
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2779;

pub fn repeated_keys_longest_run(input: &str) -> u64 {
    let samples = observe_repeated_keys(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_repeated_keys(input: &str) -> Vec<u32> {
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
        .scan(std::collections::BTreeSet::new(), |seen, (k, n)| {
            Some((seen.insert(k), n))
        })
        .filter(|(fresh, _)| !fresh)
        .map(|(_, n)| u32::from(n))
        .collect()
}
