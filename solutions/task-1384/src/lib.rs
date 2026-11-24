// rust-track: handwritten task-1384
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1384;

pub fn first_frame_byte_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_first_frame_byte(input);
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

fn observe_first_frame_byte(input: &[u8]) -> Vec<u32> {
    let mut out = vec![];
    let mut i = 0;
    while let Some(&n) = input.get(i) {
        let n = n as usize;
        let Some(payload) = input.get(i + 1..i + 1 + n) else {
            break;
        };
        let _ = payload;
        out.push(u32::from(payload.first().copied().unwrap_or(0)));
        i += 1 + n;
    }
    out
}
