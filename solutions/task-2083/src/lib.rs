// rust-track: handwritten task-2083
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2083;

pub fn four_slot_intake_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let (tx, mut rx) = tokio::sync::mpsc::channel(4);
        let mut out = vec![];
        for &x in input {
            if x == 0 {
                if let Ok(val) = rx.try_recv() {
                    out.push(val)
                }
            } else {
                let _ = tx.try_send(u32::from(x));
            }
        }
        rx.close();
        while let Ok(val) = rx.try_recv() {
            out.push(val)
        }
        out
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
