// rust-track: handwritten task-2844
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2844;

pub fn service_status_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_service_status(input);
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

fn observe_service_status(input: &[u8]) -> Vec<u32> {
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
}
