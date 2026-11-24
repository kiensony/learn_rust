// rust-track: handwritten task-2816
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2816;

pub fn service_validation_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(2);
            let reader = tokio::spawn(async move {
                let mut out = vec![];
                while let Some(msg) = rx.recv().await {
                    out.push(u32::from(msg != 0))
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
