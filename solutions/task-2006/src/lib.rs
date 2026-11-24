// rust-track: handwritten task-2006
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2006;

pub fn polled_lengths_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
