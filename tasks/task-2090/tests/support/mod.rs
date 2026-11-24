use std::future::Future;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::timeout;

pub fn run(check: impl Future<Output = ()>) {
    let rt = Builder::new_current_thread()
        .enable_time()
        .start_paused(true)
        .build()
        .unwrap();
    rt.block_on(async {
        timeout(Duration::from_secs(1), check)
            .await
            .expect("shutdown stalled with no more sends possible");
    });
}
