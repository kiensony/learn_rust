#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn frame_checksum_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut out = vec![];
            let mut i = 0;
            while let Some(&n) = input.get(i) {
                let n = n as usize;
                let Some(payload) = input.get(i + 1..i + 1 + n) else {
                    break;
                };
                let _ = payload;
                out.push(payload.iter().fold(0u32, |a, &b| a ^ u32::from(b)));
                i += 1 + n;
            }
            out
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
