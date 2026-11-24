// rust-track: handwritten task-1879
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1879;

pub fn shared_payload_longest_run(input: &[u8]) -> u64 {
    let samples = observe_shared_payload(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_shared_payload(input: &[u8]) -> Vec<u32> {
    use std::sync::Arc;
    input
        .iter()
        .map(|&x| {
            let mut data = Arc::new(x);
            let _owners: Vec<_> = (0..x % 4).map(|_| Arc::clone(&data)).collect();
            u32::from(*data)
        })
        .collect()
}
