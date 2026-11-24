#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn four_slot_queue_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        samples.sort_unstable();
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
