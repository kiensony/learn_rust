// rust-track: handwritten task-2846
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2846;

pub fn service_acknowledgments_threshold_crossing(
    input: &[u8],
    limit: u32,
) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
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
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
