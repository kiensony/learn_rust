# Review notes

`observe_early_exit_probes` returns an owned Vec<u32>. It reads the borrowed input and records early exit probes in source order. The public report takes ownership of that vector, so it can process the observations after the input borrow ends.

For a nondecreasing stream, no earlier entry is greater, so each search inspects its full prefix. Repeated equal bytes realize that quadratic cost. Output storage is linear.
