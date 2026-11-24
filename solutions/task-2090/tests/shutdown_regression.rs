mod support;
use task_2090::shutdown;
use tokio::sync::mpsc;

#[test]
fn buffered_record_precedes_a_reserved_record_during_shutdown() {
    support::run(async {
        let (tx, rx) = mpsc::channel(2);
        tx.send(30).await.unwrap();
        let permit = tx.reserve().await.unwrap();
        let receiver = tokio::spawn(shutdown(rx));
        tx.closed().await;
        permit.send(91);
        assert_eq!(receiver.await.unwrap(), [30, 91]);
    });
}
