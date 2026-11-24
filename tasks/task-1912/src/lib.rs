#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn two_slot_queue_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::sync::mpsc::sync_channel;
        let (tx, rx) = sync_channel(2);
        let mut out = vec![];
        for &x in input {
            if x == 0 {
                if let Ok(val) = rx.try_recv() {
                    out.push(val)
                }
            } else {
                let _ = tx.try_send(u32::from(x.wrapping_add(1)));
            }
        }
        drop(tx);
        out.extend(rx);
        out
    };
    samples.into_iter().map(u64::from).sum()
}
