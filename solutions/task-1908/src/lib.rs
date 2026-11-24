// rust-track: handwritten task-1908
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1908;

pub fn one_slot_queue_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::mpsc::sync_channel;
        let (tx, rx) = sync_channel(1);
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
        drop(tx);
        out.extend(rx);
        out
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
