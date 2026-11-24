#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn four_slot_intake_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let (tx, mut rx) = tokio::sync::mpsc::channel(4);
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
    let _ = samples;
    0
}
