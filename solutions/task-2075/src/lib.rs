// rust-track: handwritten task-2075
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2075;

pub fn three_slot_mailbox_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let (tx, mut rx) = tokio::sync::mpsc::channel(3);
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
    samples.dedup();
    samples
}
