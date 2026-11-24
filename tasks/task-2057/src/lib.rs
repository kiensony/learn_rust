#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn single_slot_receiver_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
