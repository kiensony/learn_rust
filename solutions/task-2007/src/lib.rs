// rust-track: handwritten task-2007
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2007;

pub fn polled_lengths_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_polled_lengths(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_polled_lengths(input: &[u8]) -> Vec<u32> {
    use std::future::Future;
    use std::task::{Context, Poll, Waker};
    let mut cx = Context::from_waker(Waker::noop());
    input
        .iter()
        .map(|&x| {
            let f = std::future::ready(u32::from(x));
            let mut pin = std::pin::pin!(f);
            match pin.as_mut().poll(&mut cx) {
                Poll::Ready(v) => v,
                Poll::Pending => unreachable!(),
            }
        })
        .collect()
}
