// rust-track: handwritten task-1915
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1915;

pub fn two_slot_queue_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::sync::mpsc::sync_channel;
        let (tx, rx) = sync_channel(2);
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
    samples.dedup();
    samples
}
