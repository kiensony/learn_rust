#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn service_acknowledgments_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
