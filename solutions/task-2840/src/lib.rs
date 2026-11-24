// rust-track: handwritten task-2840
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2840;

pub fn service_expansion_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        rt.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(2);
            let reader = tokio::spawn(async move {
                let mut out = vec![];
                while let Some(msg) = rx.recv().await {
                    out.push(u32::from(msg) * 2)
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
    samples.into_iter().rev().collect()
}
