// rust-track: handwritten task-2078
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2078;

pub fn three_slot_mailbox_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
