// rust-track: handwritten task-2067
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2067;

pub fn double_buffer_receiver_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_double_buffer_receiver(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_double_buffer_receiver(input: &[u8]) -> Vec<u32> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(2);
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
}
