mod support;
use task_2090::{TRACK_PROOF, shutdown};
use tokio::sync::mpsc;

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 2090);
}

#[test]
fn sender_already_gone() {
    support::run(async {
        let (tx, rx) = mpsc::channel(2);
        tx.send(0).await.unwrap();
        tx.send(u32::MAX).await.unwrap();
        drop(tx);
        assert_eq!(shutdown(rx).await, [0, u32::MAX]);
    });
}

#[test]
fn reservation_before_shutdown_is_committed() {
    support::run(async {
        let (tx, rx) = mpsc::channel(1);
        let permit = tx.reserve().await.unwrap();
        let worker = tokio::spawn(shutdown(rx));
        tx.closed().await;
        assert!(!worker.is_finished());
        permit.send(42);
        assert_eq!(worker.await.unwrap(), [42]);
    });
}

#[test]
fn releasing_a_reservation_allows_shutdown() {
    support::run(async {
        let (tx, rx) = mpsc::channel(1);
        let permit = tx.reserve().await.unwrap();
        let worker = tokio::spawn(shutdown(rx));
        tx.closed().await;
        drop(permit);
        assert_eq!(worker.await.unwrap(), []);
    });
}
