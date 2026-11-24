// rust-track: handwritten task-2848
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2848;

pub fn service_sharding_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
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
    };
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
