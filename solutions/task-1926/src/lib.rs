// rust-track: handwritten task-1926
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1926;

pub fn four_slot_queue_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::sync::mpsc::sync_channel;
        let (tx, rx) = sync_channel(4);
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
