// rust-track: handwritten task-2842
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2842;

pub fn service_status_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(2);
            let reader = tokio::spawn(async move {
                let mut out = vec![];
                while let Some(msg) = rx.recv().await {
                    out.push(u32::from(msg >= 128))
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
    samples.into_iter().map(u64::from).sum()
}
