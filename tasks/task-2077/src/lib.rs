#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cancel_reservation(first: u8, second: u8) -> Vec<u8> {
    use std::future::{Future, poll_fn};
    use std::task::Poll;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    runtime.block_on(async {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        tx.send(first).await.unwrap();
        let mut reservation = Box::pin(tx.reserve());
        let was_pending =
            poll_fn(|cx| Poll::Ready(reservation.as_mut().poll(cx).is_pending())).await;
        assert!(was_pending);
        drop(reservation);
        let mut delivered = Vec::new();
        let _discarded = rx.recv().await.unwrap();
        tx.send(second).await.unwrap();
        drop(tx);
        while let Some(value) = rx.recv().await {
            delivered.push(value);
        }
        delivered
    })
}
