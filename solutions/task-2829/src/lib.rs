// rust-track: handwritten task-2829
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2829;

pub fn service_sharding_longest_run(input: &[u8]) -> u64 {
    let samples = observe_service_sharding(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_service_sharding(input: &[u8]) -> Vec<u32> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(async {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(2);
        let reader = tokio::spawn(async move {
            let mut out = vec![];
            while let Some(msg) = rx.recv().await {
                out.push(u32::from(msg % 4))
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
