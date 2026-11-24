// rust-track: handwritten task-1366
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1366;

pub fn frame_offsets_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(i as u32);
            i += 1 + n;
        }
        out
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
