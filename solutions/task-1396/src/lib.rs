// rust-track: handwritten task-1396
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1396;

pub fn frame_lengths_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
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
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
