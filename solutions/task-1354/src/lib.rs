// rust-track: handwritten task-1354
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1354;

pub fn frame_lengths_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(n as u32);
            i += 1 + n;
        }
        out
    };
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
