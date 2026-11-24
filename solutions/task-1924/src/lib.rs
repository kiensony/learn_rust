// rust-track: handwritten task-1924
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1924;

pub fn four_slot_queue_peaks(input: &[u8]) -> Vec<u32> {
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
