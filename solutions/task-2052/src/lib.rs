// rust-track: handwritten task-2052
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2052;

pub fn single_slot_receiver_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
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
    samples.into_iter().map(u64::from).sum()
}
