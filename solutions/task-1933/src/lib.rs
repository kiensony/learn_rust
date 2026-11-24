// rust-track: handwritten task-1933
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1933;

pub fn finish_queue(input: &[u8]) -> Result<Vec<u8>, &'static str> {
    use std::sync::mpsc::{RecvTimeoutError, channel};
    use std::time::Duration;
    let (sender, receiver) = channel();
    for &byte in input {
        sender.send(byte).unwrap();
    }
    drop(sender);
    let mut out = Vec::new();
    loop {
        match receiver.recv_timeout(Duration::from_millis(25)) {
            Ok(value) => out.push(value),
            Err(RecvTimeoutError::Disconnected) => return Ok(out),
            Err(RecvTimeoutError::Timeout) => return Err("stalled"),
        }
    }
}
