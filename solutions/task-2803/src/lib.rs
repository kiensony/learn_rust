// rust-track: handwritten task-2803
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2803;

pub fn service_acknowledgments_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(2);
            let reader = tokio::spawn(async move {
                let mut out = vec![];
                while let Some(msg) = rx.recv().await {
                    out.push(u32::from(msg))
                }
                out
            });
            for &msg in input {
                tx.send(msg).await.unwrap()
            }
            drop(tx);
            reader.await.unwrap()
        })
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
