// rust-track: handwritten task-2090
#![forbid(unsafe_code)]

use tokio::sync::mpsc::Receiver;

pub const TRACK_PROOF: u32 = 0x52540000 | 2090;

pub async fn shutdown(mut inbox: Receiver<u32>) -> Vec<u32> {
    inbox.close();
    let mut committed = Vec::new();
    while let Some(record) = inbox.recv().await {
        committed.push(record);
    }
    committed
}
