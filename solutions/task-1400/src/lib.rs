// rust-track: handwritten task-1400
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1400;

pub fn frame_maxima_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(u32::from(payload.iter().copied().max().unwrap_or(0)));
            i += 1 + n;
        }
        out
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
