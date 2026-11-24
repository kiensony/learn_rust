// rust-track: handwritten task-1376
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1376;

pub fn frame_checksum_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
