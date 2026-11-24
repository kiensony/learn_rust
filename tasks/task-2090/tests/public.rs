mod support;
use task_2090::shutdown;
use tokio::sync::mpsc;

#[test]
fn shutdown_drains_the_whole_queue() {
    support::run(async {
        let (tx, rx) = mpsc::channel(3);
        for value in [7, 2, 7] {
            tx.send(value).await.unwrap();
        }
        assert_eq!(shutdown(rx).await, [7, 2, 7]);
        assert!(tx.send(99).await.is_err());
    });
}

#[test]
fn an_empty_queue_can_close_with_live_senders() {
    support::run(async {
        let (tx, rx) = mpsc::channel(1);
        assert!(shutdown(rx).await.is_empty());
        assert!(tx.is_closed());
    });
}
