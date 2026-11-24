#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn service_acknowledgments_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
