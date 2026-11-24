#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn five_slot_ingress_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let (tx, mut rx) = tokio::sync::mpsc::channel(5);
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
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
