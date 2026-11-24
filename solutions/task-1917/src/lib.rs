// rust-track: handwritten task-1917
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1917;

pub fn two_slot_queue_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_two_slot_queue(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_two_slot_queue(input: &[u8]) -> Vec<u32> {
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
}
