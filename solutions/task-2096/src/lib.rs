// rust-track: handwritten task-2096
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2096;

pub fn single_slot_receiver_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
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
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
