// rust-track: handwritten task-2098
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2098;

pub fn three_slot_mailbox_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
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
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
