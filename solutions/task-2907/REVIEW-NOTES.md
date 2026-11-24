# Review notes

`observe_latency_increases` returns an owned Vec<u32>. It reads the borrowed input and records latency increases in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

The regression uses the byte stream `[0, 0]`. I enumerated increasing stream lengths over bytes 0, 1, 3, 100, 200 and 255 until the original and repaired reports first differed.
