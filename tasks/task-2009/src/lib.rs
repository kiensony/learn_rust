#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn polled_lengths_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        samples.sort_unstable();
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
