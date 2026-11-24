#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn unique_owner_state_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
            use std::sync::Arc;
            input
                .iter()
                .map(|&x| {
                    let mut data = Arc::new(x);
                    let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
                    u32::from(Arc::get_mut(&mut data).is_some())
                })
                .collect()
        };
        observations.iter().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
