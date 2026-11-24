# Review notes

`observe_cumulative_occupancy` returns an owned Vec<u32>. It reads the borrowed input and records cumulative occupancy in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

Each input byte updates the running state once. The reference test recomputes from every prefix and can inspect n(n+1)/2 bytes for a stream of length n.
